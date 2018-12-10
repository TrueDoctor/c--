import re
import sys

from lexer import Lexer
from parser import Parser
from utils import print_tree

if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit('Error: no input file specified')
    with open(sys.argv[1], 'r') as cmm_file:
        code = cmm_file.read()
    name = re.sub(r'.cmm$', '', sys.argv[1])
    lex = Lexer()
    tokens = lex.tokenize(code)
    parser = Parser(tokens)
    ast = parser.parse(name)
    print_tree(ast)
