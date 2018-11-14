import re

from utils import Token


class LexerError(Exception):
    pass


class Lexer:
    types = ['int']
    control = ['if', 'else', 'while', 'for', 'return', 'inline']
    separators = ['{', '}', '(', ')', ';', ',', '=']  # currently '=' is a separator
    binary_ops = ['+', '-', '*', '/', '%', '==', '!=', '>', '>=', '<', '<=', 'or', 'and', 'not']
    keywords = ['void', *types, *control]

    def __init__(self, program):
        self.program = re.sub(r'#.*', '', program)
        regex = [
            r'(?P<op>{})'.format('|'.join(re.escape(i) for i in Lexer.ops)),
            r'(?P<sep>{})'.format('|'.join(re.escape(i) for i in Lexer.separators)),
            r'(?P<int>[0-9]+|true|false)',
            r'(?P<id>[a-zA-Z_][a-zA-Z0-9_]*)'
        ]
        self.pattern = re.compile(r'\s*(?:{})\s*'.format('|'.join(regex)))

    def tokenize(self):
        pos = 0
        line = 1
        size = len(self.program)
        while pos < size:
            match = self.pattern.match(self.program, pos)
            if not match:
                raise LexerError('Error: invalid token in line {}'.format(line))
            pos = match.end()
            line += self.program[match.start():match.end()].count('\n')
            for k, v in match.groupdict().items():
                if v:
                    if k == 'int':
                        v = int(v)
                    # if the token is an operator, a separator or a keyword
                    # k is set to v for easier parsing
                    elif k == 'id' and v in Lexer.keywords:
                        k = v
                    elif k == 'op' or k == 'sep':
                        k = v
                    yield Token(k, v, line)
                    break
