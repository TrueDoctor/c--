import os.path
import pickle

import astnode as ast
from lexer import Lexer
from parser import Parser
from code_gen import CodeGenerator
from utils import Function

# it is assumed that every function is only defined once (otherwise only the last would be used)
BUILTIN = """# builtin functions
void putchar(int arg) {
    inline <.>;
}

int getchar() {
    int _;
    inline <,>;
    return _;
}
"""
FILE_NAME = 'builtin.pkl'

def precompile():
    if os.path.exists(FILE_NAME):
        with open(FILE_NAME, 'rb') as f:
            return pickle.load(f)
    # if file doesn't exist, create it
    tokens = Lexer().tokenize(builtin)
    tree = Parser(tokens).parse('')
    stdlib = {}
    for node in tree.instr_list:
        # only consider functions
        if isinstance(node, ast.Function):
           stdlib[node.name] = Function(node)
    codegen = CodeGenerator()
    for func in stdlib.values():
        if func.code is None:
            codegen.current_funcs.append(func.node.name)
            func.code = codegen.inline_function(func.node)
            codegen.current_funcs.pop()
    with open(FILE_NAME, 'wb') as output:
        pickle.dump(stdlib, output, pickle.HIGHEST_PROTOCOL)
    return stdlib

if __name__ == '__main__':
    precompile()
