#define _CRT_SECURE_NO_DEPRECATE
#include <cstdio>
#include <iostream>
#include <vector>
#include <stack>
#include <thread>
#include <chrono>
#include <cstring>
#include <unordered_map>

using namespace std;
using bfcell = uint32_t;
using u8 = unsigned char;
using cellmap = unordered_map<int32_t, int32_t>;

enum class JumpCondition { not_zero, zero };

struct BfOpCode {
	enum BfOpCodeType {
		_nop, _inc, _dec, _rt, _lt, _set, _inp, _out, _jmp, _load, _store,
	} type;
	union {
		struct { bfcell value1; };
		struct { JumpCondition jmp_cond; uint32_t jmp_index; };
		struct { int32_t rel_pos; int32_t mult; };
	};
	BfOpCode() {  }
	BfOpCode(BfOpCodeType type) : type(type) {  }  // inp, out, store
	BfOpCode(BfOpCodeType type, bfcell cell) : type(type), value1(cell) {  }  // inc, dec, rt, lt, set
	BfOpCode(BfOpCodeType type, JumpCondition cond, uint32_t index) : type(type), jmp_cond(cond), jmp_index(index) {  }  // jmp
	BfOpCode(BfOpCodeType type, int32_t rel_pos, int32_t mult) : type(type), rel_pos(rel_pos), mult(mult) {  }  // load
};

struct BfTransducer {
	u8 *data; uint32_t len;
	vector<BfOpCode> code;
	BfTransducer(vector<u8> &raw) : data(raw.data()), len((uint32_t)raw.size()) {  }
	void push(BfOpCode c) { code.push_back(c); }
	int transduce() {
		int64_t incr = 0, shift = 0;
		stack<uint32_t> indices;
		uint32_t top;
		if (len > 2) {
			for (uint32_t i = 2; i < len; i++) {
				if (data[i] == ']' && (data[i - 1] == '-' || data[i - 1] == '+') && data[i - 2] == '[') {
					data[i] = ' ';
					data[i - 1] = '*';
					data[i - 2] = ' ';
				}
			}
		}
		for (char *pos = (char*)data; (u8*)pos != data + len; pos++) {
			char c = *(char*)pos;
			switch (c) {
			case'+': incr++; break; case'-': incr--; break; default: if (incr)
				push(BfOpCode(incr > 0 ? BfOpCode::_inc : BfOpCode::_dec, (uint32_t)abs(incr))); incr = 0;
			}
			switch (c) {
			case'>': shift++; break; case'<': shift--; break; default: if (shift)
				push(BfOpCode(shift > 0 ? BfOpCode::_rt : BfOpCode::_lt, (uint32_t)abs(shift))); shift = 0;
			}
			switch (c) {
			case '[':
				code.emplace_back(BfOpCode::_jmp, JumpCondition::zero, UINT32_MAX);
				indices.push((uint32_t)code.size());
				break;
			case ']':
				if (indices.empty()) {
					puts("error: unassigned closure");
					return -1;
				}
				top = indices.top();
				indices.pop();
				code.emplace_back(BfOpCode::_jmp, JumpCondition::not_zero, top);
				code[top - 1].jmp_index = (uint32_t)code.size();
				break;
			case '*': push(BfOpCode(BfOpCode::_set, 0)); break;
			case '.': push(BfOpCode(BfOpCode::_out)); break;
			case ',': push(BfOpCode(BfOpCode::_inp)); break;
			}
		}
		return 0;
	}
	int optimize() {
		uint32_t startindex;
		bool in_loop = false;
		int64_t shifts = 0;
		cellmap mult;
		for (uint32_t i = 0; i < code.size(); i++) {
			/*if(i==1383) {
				int dennisistdumm = true;
			}*/
			const auto op_code = code[i];
			if (op_code.type == BfOpCode::_jmp && op_code.jmp_cond == JumpCondition::zero) {
				in_loop = true;
				startindex = i;
				shifts = 0;
				mult = cellmap();
			}
			if (in_loop) {
				if (op_code.type == BfOpCode::_inp || op_code.type == BfOpCode::_out) {
					in_loop = false;
					continue;
				} else if (op_code.type == BfOpCode::_lt) {
					shifts -= op_code.value1;
				} else if (op_code.type == BfOpCode::_rt) {
					shifts += op_code.value1;
				} else if (op_code.type == BfOpCode::_inc) {
					mult[shifts] += op_code.value1;
				} else if (op_code.type == BfOpCode::_dec) {
					mult[shifts] -= op_code.value1;
				} else if (op_code.type == BfOpCode::_jmp && op_code.jmp_cond == JumpCondition::not_zero) {
					auto mf = mult.find(0);
					if (shifts != 0 || mf == mult.end()) {
						in_loop = false;
						continue;
					}
					auto m = mf->second;
					if (m == 0) puts("warning: infinite loop detected");
					if (m != -1) {
						in_loop = false;
						continue;
					}
					uint32_t n = startindex;
					code[n++] = BfOpCode(BfOpCode::_store);
					code[n++] = BfOpCode(BfOpCode::_set, 0);
					for (auto& j : mult) {
						if (j.first == 0 || j.second == 0) continue;
						code[n++] = BfOpCode(BfOpCode::_load, j.first, j.second);
					}
					for (; n <= i; n++) code[n] = BfOpCode(BfOpCode::_nop);
				}
			}
		}
		return 0;
	}
	void print() {
		for (uint32_t i = 0; i < code.size(); i++) {
			auto c = code[i];
			printf("%4d: ", i);
			switch (c.type) {
			case BfOpCode::_inc:
				printf("+ %u\n", c.value1);
				break;
			case BfOpCode::_dec:
				printf("- %u\n", c.value1);
				break;
			case BfOpCode::_rt:
				printf("> %u\n", c.value1);
				break;
			case BfOpCode::_lt:
				printf("< %u", c.value1);
			case BfOpCode::_nop:
				puts("");
				break;
			case BfOpCode::_set:
				printf("= %u\n", c.value1);
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
			case BfOpCode::_store:
				printf("x<<\n");
				break;
			}
		}
	}
	int join() {
		// ToDo: remove nops
	}
};

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
			printf("%i", i);
			if (ct == BfOpCode::_jmp) printf(" %i", *cell);
			puts("");
			switch (ct) {
			case BfOpCode::_inc: *cell += c.value1; break;
			case BfOpCode::_dec: *cell -= c.value1; break;
			case BfOpCode::_rt: cell += c.value1; break;
			case BfOpCode::_lt: cell -= c.value1; break;
			case BfOpCode::_set: *cell = c.value1; break;
			case BfOpCode::_store: storeval = *cell; break;
			case BfOpCode::_load: cell[c.rel_pos] += storeval * c.mult; break;
			case BfOpCode::_jmp: if ((c.jmp_cond == JumpCondition::zero) ^ !!*cell) i = c.jmp_index - 1; break;
			case BfOpCode::_out: putchar((int)*cell); break;
			case BfOpCode::_inp: *cell = (uint32_t)getchar(); break;
			}
		}
		return 0;
	}
};

int main(int argc, char **argv) {
	if (argc <= 1) {
		puts("interfuck is an (more or less) optimized brainfuck interpreter");
		puts("run it as follows: `interfuck script.bf`");
	}
	else if (argc >= 2) {
		vector<u8> raw;
		FILE *fhandle;
		if ((fhandle = fopen(argv[1], "rb")) == nullptr) {
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
		duc.print();
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
	}
	cin.get();
	return 0;
}
