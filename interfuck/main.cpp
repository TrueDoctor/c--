#define _CRT_SECURE_NO_WARNINGS 1

#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <cstdint>
#include <vector>
#include <stack>
#include <chrono>
#include <unordered_map>
#include <string>
#include <fcntl.h>
#include <fstream>
#include <string.h>

using namespace std;
using i8 = int8_t; using i16 = int16_t; using i32 = int32_t; using i64 = int64_t;
using u8 = uint8_t; using u16 = uint16_t; using u32 = uint32_t; using u64 = uint64_t;
using usize = size_t; using cstr = char*;

using ucell = u32; using icell = i64;

static bool show_warnings = true;

bool streq(char *a, char *b) {
	if (a == b) return true;
	for (char *i = a, *j = b; ; i++, j++) {
		if (*i != *j) return false;
		if (!*i) return true;
	}
	return true;
}

u32 strtou32(char *s) {
	u32 n = 0;
	for (char *i = s; *i; i++) {
		n *= 10;
		n += *i - '0';
	}
	return n;
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
	inline static BfInstr Nop() { return BfInstr(BfInstrCode::nop); }
	inline static BfInstr Zstore() { return BfInstr(BfInstrCode::zstore); }
	inline static BfInstr Print() { return BfInstr(BfInstrCode::print); }
	inline static BfInstr Getchr() { return BfInstr(BfInstrCode::getchr); }
	inline static BfInstr Add(icell v) { return BfInstr(BfInstrCode::add, v); }
	inline static BfInstr Shift(icell v) { return BfInstr(BfInstrCode::shift, v); }
	inline static BfInstr Set(ucell v) { return BfInstr(BfInstrCode::set, v); }
	inline static BfInstr Jump(u32 pos, bool zero) { return BfInstr(BfInstrCode::jump, JumpType{pos, zero}); }
	inline static BfInstr Load(i32 addr, icell multiplier) { return BfInstr(BfInstrCode::load, LoadType{addr, multiplier}); }
	inline static BfInstr Relset(i32 addr, ucell value) { return BfInstr(BfInstrCode::relset, RelsetType{addr, value}); }
	void print() const {
		if (type == BfInstrCode::add) printf("%c %li\n", add < 0 ? '-' : '+', add < 0 ? -add : add);
		else if (type == BfInstrCode::shift) printf("%c %li\n", shift < 0 ? '<' : '>', shift < 0 ? -shift : shift);
		else if (type == BfInstrCode::nop) puts("_none_");
		else if (type == BfInstrCode::set) printf("set %u\n", set);
		else if (type == BfInstrCode::jump) printf("jump to line %u when %sZERO\n", jump.pos, jump.zero ? "" : "NOT ");
		else if (type == BfInstrCode::zstore) puts("zstore");
		else if (type == BfInstrCode::load) printf("load to $%c%i with %lix\n", load.addr < 0 ? '-' : '+', abs(load.addr), load.multiplier);
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
			switch (c) {
				case '-': incr--; break;
				case '+': incr++; break;
				default: if (incr) { code.add(BfInstr::Add(incr)); incr = 0; }
			}
			switch (c) {
				case '<': shift--; continue;
				case '>': shift++; continue;
				default: if (shift) { code.add(BfInstr::Shift(shift)); shift = 0; }
			}
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
						if (!iter->second.reset && iter->second.change_value == 0 && show_warnings) puts("warning: infinite loop detected");
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
	static const usize memory_size = 30000;
	ucell reg, *pos, memory[memory_size];
	u32 instr_nr, code_size;
	BfRunner(BfOptCode code) : code(code) {  }
	void init() {
		memset(memory, 0, sizeof(ucell) * memory_size);
		pos = memory;
		code_size = code.size();
		instr_nr = 0;
	}
	bool step() {
		if (instr_nr >= code_size) return false;
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
		instr_nr++;
		return true;
	}
	void run() {
		init();
		while (step());
	}
};

u8 *read_file(const char *path, usize *size) {
	auto file = fopen(path, "rb");
	if (!file) return nullptr;

	if (fseek(file, 0, SEEK_END))
		return nullptr;
	i32 tell = ftell(file);
	if (tell < 0) return nullptr;
	*size = (usize)tell;
	rewind(file);
	*size -= ftell(file);

	u8 *buffer = new u8[*size];

	i32 obtained = fread(buffer, *size, 1, file);
	fclose(file);

	return obtained == 1 ? buffer : nullptr;
}

// Check if `a` contains `b` at the beginning
bool begins_with(const char *a, const char *b) {
	if (a == b) return true;
	if (a == nullptr || b == nullptr) return false;
	for (const char *i = b, *j = a; ; i++, j++) {
		if (!*i) return true;
		if (*i != *j) return false;
	}
	// This should NEVER be reached!
}

struct CommandLineArguments {
	bool help = false;
	bool version = false;
	bool debug = false;
	bool time = false;
	bool warnings = true;
	cstr filename = nullptr;
};

void print_help() {
	puts("usage: interfuck [option] ... [filename]");
	puts("  -h, --help          displays this help message");
	puts("  -v, --version       displays the current version");
	puts("  -t, --time          displays the execution time");
	puts("  -d, --debug         opens a debug console");
	puts("  -w, --no-warning    kills everyone!!");
}

void print_version() {
	puts("you really expected versioning dude?! fockin' 1.0 lol");
}

void print_debug_help() {
	puts("[required argument] {optional argument}");
	puts("following commands are available:");
	puts("  * \x1b[4mh\x1b[melp                   displays this help message");
	puts("  * \x1b[4me\x1b[mxit                   exits the application");
	puts("  * r\x1b[4mu\x1b[mn                    runs the program");
	puts("  * \x1b[4ms\x1b[mtep [n]               makes n steps in the program, default: n=1");
	puts("  * \x1b[4md\x1b[mump [n] or [s] [e]    prints the first n memory cells or the\n"
		 "                           slice from s to e");
	puts("  * \x1b[4mc\x1b[mode [n] {s}           outputs the next n code lines (starting at s)\n"
		 "                           default: s is the current code position");
	puts("  * \x1b[4mr\x1b[meset                  resets the program");
}

#define truneeu strcmp("1","12")

bool charterm(char x) { return x == ' ' || x == '\n' || !x; }

bool is_command(char *a, const char *b) {
	for (; *b; a++, b++) {
		if (*a != *b) return false;
	}
	return charterm(*a);
}

u32 argtoint(char *s) {
	u32 n = 0;
	for (char *i = s; !charterm(*i); i++) {
		n *= 10;
		n += *i - '0';
	}
	return n;
}

i32 debug(CommandLineArguments *args) {
	puts("\x1b[m");
	puts("\x1b[31m+\x1b[35m--------------------------------------\x1b[31m+\x1b[m");
	puts("\x1b[31m+ \x1b[97;1mwelcome\x1b[m\x1b[97m to the brainfuck debug shell\x1b[31m +\x1b[m");
	puts("\x1b[31m+\x1b[35m--------------------------------------\x1b[31m+\x1b[m");
	puts("");
	puts("   \x1b[1;91m*\x1b[m you can request help with '\x1b[1mhelp\x1b[m'");
	puts("   \x1b[1;91m*\x1b[m and you can exit by typing '\x1b[1mexit\x1b[m' and then press [\x1b[1menter\x1b[m]");
	puts("");
	puts("");

	usize size = 0;
	u8 *content = read_file(args->filename, &size);
	if (!content) {
		printf("error: could not read from \"%s\"\n", args->filename);
		return -1;
	}

	BfOptimizer optimizer(BfRawCode((cstr)content, (u32)size));
	BfOptCode code = optimizer.optimize();

	delete[] content;

	BfRunner runner(code);
	runner.init();

	string input;
	while (truneeu) {
		printf("$ ");
		getline(cin, input);
		char *line = (char*)input.c_str();
		vector<char*> argar;
		for (char *i = line; *i; i++) {
			if (*i == ' ') argar.emplace_back(i + 1);
		}
		if (is_command(line, "exit")) { return 0;
		} else if (is_command(line, "help")) { print_debug_help();
		} else if (is_command(line, "run")) { runner.run();
		} else if (is_command(line, "step")) {
			u32 steps = 1;
			if (argar.size() > 0) 
				steps = argtoint(argar[0]);
			for (u32 i = 0; i < steps; i++) {
				if (!runner.step()) {
					printf("-- finished execution");
					break;
				}
			}
			puts("");
		} else if (is_command(line, "reset")) {
			runner.init();
		} else if (is_command(line, "code")) {
			u32 n = 20, s = runner.instr_nr;
			if (argar.size() > 0) 
				n = argtoint(argar[0]);
			if (argar.size() > 1) 
				s = argtoint(argar[1]);
			if (s >= runner.code.size()) {
				printf("error: code is at least %u long\n", runner.code.size());
				continue;
			}
			n = min(n, (u32)runner.code.size() - runner.instr_nr);
			for (u32 i = s; i < s + n; i++) {
				printf("%04u: ", i);
				runner.code[i].print();
			}
		} else if (is_command(line, "dump")) {
			u32 n = 64, s = 0, e = 64;
			if (argar.size() > 0) {
				n = argtoint(argar[0]);
				s = runner.pos - runner.memory;
				e = s + n;
			} if (argar.size() > 1) {
				s = n;
				e = argtoint(argar[1]);
				n = e - s;
			}
			for (u32 i = s, n = 0; i < e; i++, n++) {
				if (n && !(n % 16)) puts("");
				printf("%02x ", runner.memory[i]);
			}
			puts("");
		} else {
			puts("error: command not found");
		}
	}

	return 0;
}

i32 main(i32 argc, cstr argv[]) {
	CommandLineArguments args;

	for (u32 i = 1; i < argc; i++) {
		cstr arg = argv[i];
		switch (*arg) {
			case 0: continue;
			case '-': break;
			default: args.filename = arg;
					 continue;
		}
		if (*++arg == '-') {
			arg++;
			if (begins_with(arg, "help")) args.help = true;
			else if (begins_with(arg, "version")) args.version = true;
			else if (begins_with(arg, "debug")) args.debug = true;
			else if (begins_with(arg, "time")) args.time = true;
			else if (begins_with(arg, "no-warning")) args.warnings = false;
			else printf("warning: argument '--%s' is unknown; skipping argument\n", arg);
		} else {
			do {
				switch (*arg) {
					case 'h':
						args.help = true;
						break;
					case 'v':
						args.version = true;
						break;
					case 'd':
						args.debug = true;
						break;
					case 't':
						args.time = true;
						break;
					case 'w':
						args.warnings = false;
						break;
					default:
						printf("warning: argument '-%c' is unknown; skipping argument\n", *arg);
				}
			} while (*++arg);
		}
	}

	show_warnings = args.warnings;

	if (args.help) {
		print_help();
		return 0;
	}

	if (args.version) {
		print_version();
		return 0;
	}

   	if (args.debug)
		return debug(&args);

	if (!args.filename || !*args.filename) {
		puts("error: needing an input file");
		return -1;
	}

	usize size = 0;
	u8 *content = read_file(args.filename, &size);
	if (!content) {
		printf("error: could not read from \"%s\"\n", args.filename);
		return -1;
	}

	BfOptimizer optimizer(BfRawCode((cstr)content, (u32)size));
	BfOptCode code = optimizer.optimize();

	delete[] content;

	BfRunner runner(code);

	if (args.time) {
		auto t0 = chrono::high_resolution_clock::now();
		runner.run();
		auto dt = chrono::high_resolution_clock::now() - t0;
		printf("---\nthe code took %llims\n", chrono::duration_cast<chrono::milliseconds>(dt).count());
	} else runner.run();

	return 0;
}
