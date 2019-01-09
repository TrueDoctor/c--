import sys

from lexer import Lexer
from parser import Parser
from code_gen import CodeGenerator
from utils import print_tree, CompilerError

if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit('Error: no input file specified')
    with open(sys.argv[1], 'r') as cmm_file:
        code = cmm_file.read()
    try:
        lex = Lexer()
        tokens = lex.tokenize(code)
        parser = Parser(tokens)
        tree = parser.parse(sys.argv[1])
        code_generator = CodeGenerator(tree)
        code = code_generator.generate()
        print_tree(tree)
        print()
        print(code)
    except CompilerError as e:
        sys.exit(e)
