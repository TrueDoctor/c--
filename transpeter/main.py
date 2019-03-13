import sys
import os.path
import pickle
import hashlib
from argparse import ArgumentParser

from lexer import Lexer
from parser import Parser
from code_gen import CodeGenerator
from utils import print_tree, CompilerError

path = os.path.dirname(__file__)
stdlib_src = os.path.join(path, 'std.lib')


def load_stdlib():
    try:
        with open(stdlib_src) as stdlib_src_file:
            stdlib_src_code = stdlib_src_file.read()
    except OSError as e:
        print('an error occured while loading stdlib', file=sys.stderr)
        sys.exit(e)
    file_name = os.path.join(path, 'stdlib_' + hashlib.md5(stdlib_src_code.encode()).hexdigest()[0:8] + '.pkl')
    if os.path.exists(file_name) and not args.recompile:
        with open(file_name, 'rb') as f:
            return pickle.load(f)
    else:
        lexer = Lexer()
        tokens = lexer.tokenize(stdlib_src_code)
        tree = Parser(tokens).parse('stdlib')
        code_generator = CodeGenerator(tree)
        code_generator.generate()
        stdlib = code_generator.functions
        with open(file_name, 'wb') as f:
            pickle.dump(stdlib, f, pickle.HIGHEST_PROTOCOL)
        return stdlib


if __name__ == "__main__":
    arg_parser = ArgumentParser()
    arg_parser.add_argument('-d', '--debug', help='prints stack trace for errors', action='store_true')
    arg_parser.add_argument('-t', '--tree', help='prints the abstract syntax tree', action='store_true')
    arg_parser.add_argument('-o', '--optimize', help='optimizes the emitted code', action='store_true')
    arg_parser.add_argument('-r', '--recompile', help='recompiles the standard library', action='store_true')
    arg_parser.add_argument('src', help='source file')
    arg_parser.add_argument('dest', help='destination file', nargs='?', default=None)
    args = arg_parser.parse_args()
    try:
        with open(args.src) as cmm_file:
            code = cmm_file.read()
    except OSError as e:
        print(e, file=sys.stderr)
        sys.exit(arg_parser.format_usage())
    try:
        lex = Lexer()
        stdlib = load_stdlib()
        tokens = lex.tokenize(code)
        parser = Parser(tokens)
        tree = parser.parse(os.path.basename(args.src))
        code_generator = CodeGenerator(tree, stdlib)
        code = code_generator.generate(args.optimize)
        if args.tree:
            print_tree(tree)
            if args.dest is None:
                print()
        if args.dest is not None:
            try:
                with open(args.dest, 'w') as out_file:
                    out_file.write(code)
            except OSError as e:
                print(e, file=sys.stderr)
                sys.exit(arg_parser.format_usage())
        else:
            print(code)
    except CompilerError as e:
        if args.debug:
            raise e
        sys.exit(e)
