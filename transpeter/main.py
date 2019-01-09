import sys

from lexer import Lexer
from parser import Parser
from code_gen import CodeGenerator
from utils import print_tree

if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit('Error: no input file specified')
    with open(sys.argv[1], 'r') as cmm_file:
        code = cmm_file.read()
    lex = Lexer()
    tokens = lex.tokenize(code)
    parser = Parser(tokens)
    tree = parser.parse(sys.argv[1])
    print_tree(tree)
    code_generator = CodeGenerator(tree)
    print()
    print(code_generator.generate())
