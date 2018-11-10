import re
import sys


class Lexer:
    types = ['void', 'int']
    control = ['if', 'else', 'while', 'for', 'return', 'inline']
    binary_ops = ['+', '-', '*', '/', '%', '==', '!=', '>', '>=', '<', '<=', '|', '&']
    unary_ops = ['!']
    ops = [*unary_ops, *binary_ops]
    separators = ['{', '}', '(', ')', ';', ',', '=']
    keywords = [*types, *control]
    
    def __init__(self, program):
        self.program = re.sub(r'#.*', '', program)
        regex = [
            r'(?P<op>{})'.format('|'.join(re.escape(i) for i in Lexer.ops)),
            r'(?P<sep>{})'.format('|'.join(re.escape(i) for i in Lexer.separators)),
            r'(?P<int>[0-9]+)',
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
                sys.exit('Error: invalid token in line {}'.format(line))
            pos = match.end()
            line += self.program[match.start():match.end()].count('\n')
            for k, v in match.groupdict().items():
                if v:
                    if k == 'int':
                        v = int(v)
                    elif k == 'id' and v in self.keywords:
                        k = 'key'
                    yield Token(k, v, line)
                    break


class Token:
    def __init__(self, token_type, value, line):
        self.type = token_type
        self.value = value
        self.line = line

    def __repr__(self):
        return '{}: {}'.format(self.type, self.value)
