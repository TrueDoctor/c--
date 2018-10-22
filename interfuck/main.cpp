#define _CRT_SECURE_NO_WARNINGS 1

#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <cstdint>
#include <vector>
#include <stack>
#include <chrono>
#include <unordered_map>
#include <io.h>
#include <fcntl.h>

using namespace std;
using i8 = int8_t; using i16 = int16_t; using i32 = int32_t; using i64 = int64_t;
using u8 = uint8_t; using u16 = uint16_t; using u32 = uint32_t; using u64 = uint64_t;
using usize = size_t; using cstr = char*;

using ucell = u32; using icell = i64;

void print_help() {
	puts("usage:");
	puts("interfuck [filename]");
}

enum class BfInstrCode : u32 {
	nop, add, shift, set, jump, zstore, load, relset, print, getchr,
};

struct BfInstr {
	struct JumpType { u32 pos; bool zero; };
	struct LoadType { i32 addr; icell multiplier; };
	struct RelsetType { i32 addr; ucell value; };
	BfInstrCode type;
	union {
		icell add;
		icell shift;
		ucell set;
		JumpType jump;
		LoadType load;
		RelsetType relset;
	};
	BfInstr() {  }
	BfInstr(BfInstrCode type) : type(type) {  }
	BfInstr(BfInstrCode type, icell x) : type(type), add(x) {  }
	BfInstr(BfInstrCode type, ucell x) : type(type), set(x) {  }
	BfInstr(BfInstrCode type, JumpType x) : type(type), jump(x) {  }
	BfInstr(BfInstrCode type, LoadType x) : type(type), load(x) {  }
	BfInstr(BfInstrCode type, RelsetType x) : type(type), relset(x) {  }
	inline static BfInstr Nop() { return BfInstr(BfInstrCode::nop); } inline static BfInstr Zstore() { return BfInstr(BfInstrCode::zstore); }
	inline static BfInstr Print() { return BfInstr(BfInstrCode::print); } inline static BfInstr Getchr() { return BfInstr(BfInstrCode::getchr); }
	inline static BfInstr Add(icell v) { return BfInstr(BfInstrCode::add, v); }
	inline static BfInstr Shift(icell v) { return BfInstr(BfInstrCode::shift, v); }
	inline static BfInstr Set(ucell v) { return BfInstr(BfInstrCode::set, v); }
	inline static BfInstr Jump(u32 pos, bool zero) { return BfInstr(BfInstrCode::jump, JumpType{pos, zero}); }
	inline static BfInstr Load(i32 addr, icell multiplier) { return BfInstr(BfInstrCode::load, LoadType{addr, multiplier}); }
	inline static BfInstr Relset(i32 addr, ucell value) { return BfInstr(BfInstrCode::relset, RelsetType{addr, value}); }
	void print() const {
		if (type == BfInstrCode::add) printf("%c %lli\n", add < 0 ? '-' : '+', add < 0 ? -add : add);
		else if (type == BfInstrCode::shift) printf("%c %lli\n", shift < 0 ? '<' : '>', shift < 0 ? -shift : shift);
		else if (type == BfInstrCode::nop) puts("_none_");
		else if (type == BfInstrCode::set) printf("set %u\n", set);
		else if (type == BfInstrCode::jump) printf("jump to line %u when %sZERO\n", jump.pos, jump.zero ? "" : "NOT ");
		else if (type == BfInstrCode::zstore) puts("zstore");
		else if (type == BfInstrCode::load) printf("load to $%c%i with %ix\n", load.addr < 0 ? '-' : '+', abs(load.addr), load.multiplier);
		else if (type == BfInstrCode::relset) printf("set to $%c%i value %i\n", relset.addr < 0 ? '-' : '+', abs(relset.addr), relset.value);
		else if (type == BfInstrCode::print) puts("print");
		else if (type == BfInstrCode::getchr) puts("getchr");
	}
};

struct LoadProperty {
	bool reset; ucell reset_value; icell change_value;
	inline static LoadProperty Change(icell x) { return { false, 0, x }; }
	inline static LoadProperty Set(ucell x) { return { true, x, 0 }; }
};
using cellmap = unordered_map<i32, LoadProperty>;

struct BfRawCode {
	cstr code; u32 size;
	BfRawCode(cstr code, u32 size) : code(code), size(size) { }
	inline char operator[](u32 n) const { return code[n]; }
	inline void set(u32 n, char x) { code[n] = x; }
};

struct BfOptCode : vector<BfInstr> {
	BfOptCode() {  }
	inline void add(BfInstr x) { emplace_back(x); }
	void print() {
		u32 n = 0;
		for (auto &i : *this) {
			printf("%4u: ", n++);
			i.print();
		}
	}
	void remove_nops() {
		BfOptCode old;
		old.resize(this->size());
		vector<u32> indices(old.size());
		for (u32 i = 0; i < this->size(); i++) old[i] = (*this)[i];
		u32 n = 0;
		for (u32 i = 0; i < old.size(); i++) {
			auto c = old[i];
			indices[i] = n;
			if (c.type != BfInstrCode::nop) n++;
		}
		clear();
		resize(n);
		n = 0;
		for (u32 i = 0; i < old.size(); i++) {
			auto c = old[i];
			if (c.type != BfInstrCode::nop) {
				if (c.type == BfInstrCode::jump)
					c.jump.pos = indices[c.jump.pos];
				(*this)[n++] = c;
			}
		}
	}
};

struct BfOptimizer {
	BfRawCode raw_code;
	BfOptimizer(BfRawCode raw_code) : raw_code(raw_code) { }
	BfOptCode to_opt() {
		BfOptCode code;
		stack<u32> pos_stack;
		icell incr = 0;
		i32 shift = 0;
		for (u32 i = 2; i < raw_code.size; i++) {
			if (raw_code[i] == '*') raw_code.set(i, ' ');
			if (raw_code[i] == ']' && (raw_code[i - 1] == '-' || raw_code[i - 1] == '+') && raw_code[i - 2] == '[') {
				raw_code.set(i, ' ');
				raw_code.set(i - 1, '*');
				raw_code.set(i - 2, ' ');
			}
		}
		for (u32 i = 0; i < raw_code.size; i++) {
			auto c = raw_code[i];
			switch (c) { case '-': incr--; break; case '+': incr++; break;
			default: if (incr) { code.add(BfInstr::Add(incr)); incr = 0; } }
			switch (c) { case '<': shift--; continue; case '>': shift++; continue;
			default: if (shift) { code.add(BfInstr::Shift(shift)); shift = 0; } }
			if (c == '.') code.add(BfInstr::Print());
			else if (c == ',') code.add(BfInstr::Getchr());
			else if (c == '[') {
				code.add(BfInstr::Jump(0, true));
				pos_stack.push((u32)code.size());
			} else if (c == ']') {
				if (pos_stack.empty()) {
					puts("error: unopening bracket");
					exit(-1);
				}
				auto jump_pos = pos_stack.top();
				pos_stack.pop();
				code.add(BfInstr::Jump(jump_pos, false));
				code[jump_pos - 1].jump.pos = (u32)code.size();
			}
			else if (c == '*')
				code.add(BfInstr::Set(0));
		}
		if (!pos_stack.empty()) {
			puts("error: unclosing bracket");
			exit(-1);
		}
		return code;
	}
	void optimize(BfOptCode &code) const {
		u32 csz = (u32)code.size();
		for (u32 i = 0; i < csz; i++) {
			auto c1 = code[i];
			if (c1.type == BfInstrCode::jump && c1.jump.zero) {
				u32 shift = 0;
				cellmap multiplier;
				u32 j = i + 1;
				for (; j < csz; j++) {
					auto c = code[j];
					if (c.type == BfInstrCode::shift) shift += c.shift;
					else if (c.type == BfInstrCode::add) {
						auto f = multiplier.find(shift);
						if (f == multiplier.end()) multiplier.emplace(shift, LoadProperty::Change(c.add));
						else f->second.change_value += c.add;
					} else if (c.type == BfInstrCode::print || c.type == BfInstrCode::getchr)
						break;
					else if (c.type == BfInstrCode::set) {
						auto f = multiplier.find(shift);
						if (f == multiplier.end()) multiplier.emplace(shift, LoadProperty::Set(c.set));
						else f->second = LoadProperty::Set(c.set);
					} else if (c.type == BfInstrCode::jump) {
						if (c.jump.zero) break;
						auto iter = multiplier.find(0);
						if (iter == multiplier.end()) break;
						if (!iter->second.reset && iter->second.change_value == 0) puts("warning: infinite loop detected");
						if (iter->second.reset || iter->second.change_value != -1) break;
						u32 n = i;
						code[++n] = BfInstr::Zstore();
						for (auto &p : multiplier) {
							if (p.first == 0) continue;
							if (p.second.reset)
								code[++n] = BfInstr::Relset(p.first, p.second.reset_value);
							if (p.second.change_value != 0)
								code[++n] = BfInstr::Load(p.first, p.second.change_value);
						}
						for (n++; n <= j; ) code[n++] = BfInstr::Nop();
						break;
					}
				}
				i = j;
			}
		}
	}
	BfOptCode optimize() {
		BfOptCode code = to_opt();
		optimize(code);
		code.remove_nops();
		return code;
	}
};

struct BfRunner {
	BfOptCode code;
	BfRunner(BfOptCode code) : code(code) {  }
	void run() {
		const usize memory_size = 4096;
		ucell memory[memory_size];
		memset(memory, 0, sizeof(ucell) * memory_size);
		ucell *pos = memory;
		ucell reg;
		u32 code_size = code.size();
		for (u32 instr_nr = 0; instr_nr < code_size; instr_nr++) {
			auto c = code[instr_nr];
			switch (c.type) {
			case BfInstrCode::add:
				*pos += c.add; break;
			case BfInstrCode::shift:
				pos += c.shift; break;
			case BfInstrCode::set:
				*pos = c.set; break;
			case BfInstrCode::zstore:
				reg = *pos; *pos = 0; break;
			case BfInstrCode::load:
				pos[c.load.addr] += reg * c.load.multiplier; break;
			case BfInstrCode::jump:
				if ((!*pos && c.jump.zero) || (*pos && !c.jump.zero)) instr_nr = c.jump.pos - 1;
				break;
			case BfInstrCode::relset:
				pos[c.relset.addr] = c.relset.value; break;
			case BfInstrCode::print:
				putchar(*pos); break;
			case BfInstrCode::getchr:
				*pos = getchar(); break;
			}
		}
	}
};

i32 main(i32 argc, cstr argv[]) {
	if (argc <= 1) {
		puts("error: needing at least one argument");
		print_help();
		return -1;
		/* argc = 2;
		argv = new cstr[2]{ argv[0], (cstr)"mandelbrot.bf" };*/
	}
	cstr filename = argv[argc - 1];
	i32 file = _open(filename, _O_BINARY | _O_RDWR);
	if (file < 0) {
		printf("error: could not open the file '%s'\n", filename);
		return -1;
	}
	auto raw_size = _filelengthi64(file);
	u8 *buf = new u8[raw_size];
	if (_read(file, buf, (u32)raw_size) < 0) {
		printf("error: could not read in the file '%s'\n", filename);
		return -1;
	}
	_close(file);

	BfOptimizer optimizer(BfRawCode((cstr)buf, (u32)raw_size));
	BfOptCode code = optimizer.optimize();
	// code.print();

	delete[] buf;

	BfRunner runner(code);

	auto t0 = chrono::high_resolution_clock::now();
	runner.run();
	auto dt = chrono::high_resolution_clock::now() - t0;
	printf("---\nthe code took %llims", chrono::duration_cast<chrono::milliseconds>(dt).count());

	cin.get();

	return 0;
}