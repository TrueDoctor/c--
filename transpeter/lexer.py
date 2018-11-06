import re
import sys


class Lexer:
    def __init__(self, program):
        self.program = program
        self.keywords = []
        regex = [
            r'(?P<sep>\(|\)|{|}|;|,|\s+)',
            r'(?P<op>=|\+|-|\*|/|%)',
            r'(?P<int>[0-9]+)',
            r'(?P<id>[a-zA-Z_][a-zA-Z0-9_]*)'
        ]
        self.pattern = re.compile('|'.join(regex))

    def tokenize(self):
        tokens = []
        line = 0
        self.program = re.sub(r'#.*', '', self.program)
        while self.program:
            match = self.pattern.match(self.program)
            if not match:
                sys.exit('Error: invalid token in line {}'.format(line))
            line += self.program[:match.end()].count('\n')
            self.program = self.program[match.end():]
            for k, v in match.groupdict().items():
                if v:
                    if k == 'int':
                        v = int(v)
                    elif k == 'sep':
                        v = re.sub(r'\s+', ' ', v)
                    elif k == 'id' and v in self.keywords:
                        k = 'key'
                    tokens.append(Token(k, v, line))
        return tokens


class Token:
    def __init__(self, type, value, line):
        self.type = type
        self.value = value
        self.line = line

    def __repr__(self):
        return '{}: {}'.format(self.type, self.value)
