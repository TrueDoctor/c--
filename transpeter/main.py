from re import sub
from sys import argv, exit

from lexer import Lexer
from parser import Parser
from utils import print_tree

if __name__ == "__main__":
    if len(argv) < 2:
        exit('Error: no input file specified')
    with open(argv[1], 'r') as file:
        code = file.read()
    name = sub(r'.cmm$', '', argv[1])
    lex = Lexer()
    tokens = lex.tokenize(code)
    parser = Parser(tokens)
    ast = parser.parse(name)
    print_tree(ast)
