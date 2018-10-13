#include <cstdio>
#include <cstddef>
#include <iostream>
#include <vector>
#include <utility>
#include <stack>
#include <thread>
#include <chrono>

using namespace std;
using bfcell = uint32_t;
using byte = unsigned char;

enum class JumpCondition { not_zero, zero };

struct BfOpCode {
	enum BfOpCodeType {
		_inc, _dec, _rt, _lt, _set, _inp, _out, _jmp,
	} type;
	union {
		struct { bfcell cell1; };
		struct { JumpCondition jmp_cond; uint32_t jmp_index; };
	};
	BfOpCode() {  }
	BfOpCode(BfOpCodeType type) : type(type) {  }
	BfOpCode(BfOpCodeType type, bfcell cell) : type(type), cell1(cell) {  }
	BfOpCode(BfOpCodeType type, JumpCondition cond, uint32_t index) : type(type), jmp_cond(cond), jmp_index(index) {  }
};

struct BfTransducer {
	byte *data; uint32_t len;
	vector<BfOpCode> code;
	BfTransducer(vector<byte> &raw) : data(raw.data()), len((uint32_t)raw.size()) {  }
	void push(BfOpCode c) { code.push_back(c); }
	int transduce() {
		int64_t incr = 0, shift = 0;
		stack<uint32_t> indices;
		uint32_t top;
		for (char *pos = (char*)data; (byte*)pos != data + len; pos++) {
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
				code.push_back(BfOpCode(BfOpCode::_jmp, JumpCondition::zero, UINT32_MAX));
				indices.push((uint32_t)code.size());
				break;
			case ']':
				if (!indices.size()) {
					puts("error: unassigned closure");
					return -1;
				}
				top = indices.top();
				indices.pop();
				code.push_back(BfOpCode(BfOpCode::_jmp, JumpCondition::not_zero, top));
				code[top - 1].jmp_index = (uint32_t)code.size();
				break;
			case '.': push(BfOpCode(BfOpCode::_out)); break;
			case ',': push(BfOpCode(BfOpCode::_inp)); break;
			}
		}
		return 0;
	}
};

struct BfVirtualEnv {
	BfOpCode *code; uint32_t len;
	BfVirtualEnv(vector<BfOpCode> &code) : code(code.data()), len((uint32_t)code.size()) {  }
	int run() {
		uint32_t memory[4096];
		memset(memory, 0, sizeof(memory));
		uint32_t *cell = memory;
		for (uint32_t i = 0; i < len; i++) {
			BfOpCode c = code[i];
			switch (c.type) {
			case BfOpCode::_inc: *cell += c.cell1; break;
			case BfOpCode::_dec: *cell -= c.cell1; break;
			case BfOpCode::_rt: cell += c.cell1; break;
			case BfOpCode::_lt: cell -= c.cell1; break;
			case BfOpCode::_out: putchar((int)*cell); break;
			case BfOpCode::_inp: *cell = (uint32_t)getchar(); break;
			case BfOpCode::_jmp: if ((c.jmp_cond == JumpCondition::zero) ^ !!*cell) i = c.jmp_index - 1; break;
			}
		}
		return 0;
	}
};

int main(int argc, char **argv) {
	errno_t res;
	if (argc <= 1) {
		puts("interfuck is an (more or less) optimized brainfuck interpreter");
		puts("run it as follows: ´interfuck script.bf´");
	}
	else if (argc >= 2) {
		vector<byte> raw;
		FILE *fhandle;
		if (res = fopen_s(&fhandle, argv[1], "rb")) {
			printf("error: file open failed with os errno %i\n", res);
			return -1;
		}
		int c;
		while ((c = fgetc(fhandle)) != EOF)
			switch (c) { case'+':case'-':case'>':case'<':case'[':case']':case'.':case',':raw.push_back(byte(c)); }
		// for (uint32_t i = 0; i < raw.size(); i++) putchar((int)raw[i]);
		// putchar('\n');
		raw.shrink_to_fit();
		BfTransducer duc(raw);
		if (duc.transduce()) {
			puts("error: error in bf file");
			return -1;
		}
		BfVirtualEnv env(duc.code);
		chrono::high_resolution_clock myclock;
		auto t0 = myclock.now();
		if (env.run()) {
			puts("error: error while executing code in the virtual env");
			return -1;
		}
		auto dt = myclock.now() - t0;
		puts("\n---------------");
		printf("execution time: %llims", chrono::duration_cast<chrono::milliseconds>(dt).count());
	}
	return 0;
}