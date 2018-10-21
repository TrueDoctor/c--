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

<<<<<<< HEAD
struct BfOpCode {
	enum BfOpCodeType {
		_nop, _inc, _dec, _rt, _lt, _set, _inp, _out, _jmp, _load, _zstore,
	} type;
	union {
		struct { bfcell cell1; };
		struct { JumpCondition jmp_cond; uint32_t jmp_index; };
		struct { int32_t rel_pos; int32_t mult; };
	};
	BfOpCode() {  }
	BfOpCode(BfOpCodeType type) : type(type) {  }  // inp, out, store
	BfOpCode(BfOpCodeType type, bfcell cell) : type(type), cell1(cell) {  }  // inc, dec, rt, lt, set
	BfOpCode(BfOpCodeType type, JumpCondition cond, uint32_t index) : type(type), jmp_cond(cond), jmp_index(index) {  }  // jmp
	BfOpCode(BfOpCodeType type, int32_t rel_pos, int32_t mult) : type(type), rel_pos(rel_pos), mult(mult) {  }  // load
=======
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
>>>>>>> d128c748502e5b2df2a100a95e72817f061789df
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
<<<<<<< HEAD
	int optimize() {
		uint32_t startindex;
		bool in_loop = false;
		int64_t shifts = 0;
		cellmap mult;
		vector<int> sets;
		for (uint32_t i = 0; i < code.size(); i++) {
			auto c = code[i];
			if (c.type == BfOpCode::_jmp && c.jmp_cond == JumpCondition::zero) {
				in_loop = true;
				startindex = i;
				shifts = 0;
				mult = cellmap();
				sets = vector<int>();
			}
			if (in_loop) {
				if (c.type == BfOpCode::_inp || c.type == BfOpCode::_out) {
					in_loop = false;
					continue;
				} else if (c.type == BfOpCode::_lt) {
					shifts -= c.cell1;
				} else if (c.type == BfOpCode::_rt) {
					shifts += c.cell1;
				} else if (c.type == BfOpCode::_inc) {
					mult[shifts] += c.cell1;
				} else if (c.type == BfOpCode::_dec) {
					mult[shifts] -= c.cell1;
				} else if (c.type == BfOpCode::_set) {
					sets.emplace_back(shifts);
				} else if (c.type == BfOpCode::_jmp && c.jmp_cond == JumpCondition::not_zero) {
					auto mf = mult.find(0);
					if (mf == mult.end()) {
						in_loop = false;
						continue;
					}
					auto m = mf->second;
					if (m == 0) puts("warning: infinite loop detected");
					if (shifts != 0 || m != -1) {
						in_loop = false;
						continue;
					}
					uint32_t n = startindex+1;
					code[n++] = BfOpCode(BfOpCode::_zstore);
					//code[n++] = BfOpCode(BfOpCode::_set, 0);
					for (auto j = mult.begin(); j != mult.end(); j++) {
						if (j->first == 0 || j->second == 0) continue;
						code[n++] = BfOpCode(BfOpCode::_load, j->first, j->second);
					}
					for (std::_Vector_iterator<std::_Vector_val<std::_Simple_types<int>>>::value_type& set : sets)
					{
						code[n++] = BfOpCode(BfOpCode::_set, set, 0);
					}
					for (; n <= i; n++) code[n] = BfOpCode(BfOpCode::_nop);
					in_loop = false;
=======
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
>>>>>>> d128c748502e5b2df2a100a95e72817f061789df
				}
				auto jump_pos = pos_stack.top();
				pos_stack.pop();
				code.add(BfInstr::Jump(jump_pos, false));
				code[jump_pos - 1].jump.pos = (u32)code.size();
			}
			else if (c == '*')
				code.add(BfInstr::Set(0));
		}
		return code;
	}
<<<<<<< HEAD
	void print() {
		for (uint32_t i = 0; i < code.size(); i++) {
			auto c = code[i];
			printf("%4d: ", i);
			switch (c.type) {
			case BfOpCode::_inc:
				printf("+ %u\n", c.cell1);
				break;
			case BfOpCode::_dec:
				printf("- %u\n", c.cell1);
				break;
			case BfOpCode::_rt:
				printf("> %u\n", c.cell1);
				break;
			case BfOpCode::_lt:
				printf("< %u", c.cell1);
			case BfOpCode::_nop:
				puts("");
				break;
			case BfOpCode::_set:
				printf("= [%i]*%i\n", c.rel_pos, c.mult);
				break;
			case BfOpCode::_inp:
				printf("cin<<\n");
				break;
			case BfOpCode::_out:
				printf("cout>>\n");
				break;
			case BfOpCode::_jmp:
				printf("=> %c0 | %u\n", c.jmp_cond == JumpCondition::zero ? '=' : '!', c.jmp_index);
				break;
			case BfOpCode::_load:
				printf("x>>[%i]*%i\n", c.rel_pos, c.mult);
				break;
			case BfOpCode::_zstore:
				printf("x<<\n");
				break;
=======
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
>>>>>>> d128c748502e5b2df2a100a95e72817f061789df
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

<<<<<<< HEAD
struct BfVirtualEnv {
	BfOpCode *code; uint32_t len;
	BfVirtualEnv(vector<BfOpCode> &code) : code(code.data()), len((uint32_t)code.size()) {  }
	int run() const {
		uint32_t memory[4096];
		memset(memory, 0, sizeof(memory));
		uint32_t *cell = memory;
		bfcell storeval;
		for (uint32_t i = 0; i < len; i++) {
			const BfOpCode c = code[i];
			const BfOpCode::BfOpCodeType ct = c.type;
			switch (ct) {
			case BfOpCode::_inc: *cell += c.cell1; break;
			case BfOpCode::_dec: *cell -= c.cell1; break;
			case BfOpCode::_rt: cell += c.cell1; break;
			case BfOpCode::_lt: cell -= c.cell1; break;
			case BfOpCode::_set: cell[c.rel_pos] = c.mult; break;
			case BfOpCode::_zstore: storeval = *cell; *cell = 0; break;
			case BfOpCode::_load: cell[c.rel_pos] += storeval * c.mult; break;
			case BfOpCode::_jmp: if ((c.jmp_cond == JumpCondition::zero) ^ !!*cell) i = c.jmp_index - 1; break;
			case BfOpCode::_out: putchar((int)*cell); break;
			case BfOpCode::_inp: *cell = (uint32_t)getchar(); break;
=======
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
>>>>>>> d128c748502e5b2df2a100a95e72817f061789df
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
<<<<<<< HEAD
	else if (argc >= 2) {
		vector<u8> raw;
		FILE *fhandle;
		if ((fhandle = fopen(argv[1], "rb")) == NULL) {
			printf("error: file open failed! Are you sure that %s exists?\n", argv[1]);
			return -1;
		}
		int c;
		while ((c = fgetc(fhandle)) != EOF)
			switch (c) { case'+':case'-':case'>':case'<':case'[':case']':case'.':case',':raw.push_back(u8(c)); }
		// for (uint32_t i = 0; i < raw.size(); i++) putchar((int)raw[i]);
		// putchar('\n');
		raw.shrink_to_fit();
		BfTransducer duc(raw);
		if (duc.transduce()) {
			puts("error: error in bf file");
			return -1;
		}
		if (duc.optimize()) {
			puts("error: error while optimizing bf file");
			return -1;
		}
		//duc.print();
		BfVirtualEnv env(duc.code);
		const auto t0 = chrono::high_resolution_clock::now();
		if (env.run()) {
			puts("error: error while executing code in the virtual env");
			return -1;
		}
		const auto dt = chrono::high_resolution_clock::now() - t0;
		puts("\n---------------");
		printf("execution time: %llims", chrono::duration_cast<chrono::milliseconds>(dt).count());
		fclose(fhandle);
=======
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
>>>>>>> d128c748502e5b2df2a100a95e72817f061789df
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