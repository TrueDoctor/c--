import sys
import os.path
from argparse import ArgumentParser

from lexer import Lexer
from parser import Parser
from code_gen import CodeGenerator
from utils import print_tree, CompilerError

if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('-d', '--debug', help='prints stack trace for errors', action='store_true')
    parser.add_argument('-t', '--tree', help='prints the abstract syntax tree', action='store_true')
    parser.add_argument('-o', '--optimize', help='optimizes the emitted code', action='store_true')
    parser.add_argument('src', help='source file')
    parser.add_argument('dest', help='destination file', nargs='?', default=None)
    args = parser.parse_args()
    try:
        with open(args.src) as cmm_file:
            code = cmm_file.read()
    except OSError as e:
        print(e, file=sys.stderr)
        sys.exit(parser.format_usage())
    try:
        lex = Lexer()
        tokens = lex.tokenize(code)
        parser = Parser(tokens)
        tree = parser.parse(os.path.basename(args.src)
        code_generator = CodeGenerator(tree)
        code = code_generator.generate(optimize=args.optimize)
        if args.tree:
            print_tree(tree)
            if args.dest is None:
                print()
        if args.dest is not None:
            with open(args.dest, 'w') as out_file:
                out_file.write(code)
        else:
            print(code)
    except CompilerError as e:
        if args.debug:
            raise e
        sys.exit(e)
