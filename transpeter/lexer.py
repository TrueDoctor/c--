import re

from utils import Token


class LexerError(Exception):
    pass


class Lexer:
    ops = ['+=', '+', '-=', '-', '*=', '*', '/=', '/', '%=', '%', '==', '!=', '>=', '<=', '>', '<', 'or', 'and', 'not']
    separators = ['=', '{', '}', '(', ')', ';', ',']
    types = ['void', 'int']
    control = ['if', 'else', 'while', 'for', 'return', 'inline']
    escape = [r'\n', r'\r', r'\t', r'\b']

    def __init__(self):
        regex = [
            r'(?P<op>{})'.format('|'.join(re.escape(i) for i in Lexer.ops)),
            r'(?P<sep>{})'.format('|'.join(re.escape(i) for i in Lexer.separators)),
            r'(?P<int>[0-9]+|true|false)',
            r'\'(?P<char>.|{})\''.format('|'.join(re.escape(i) for i in Lexer.escape)),
            r'(?P<id>[a-zA-Z_][a-zA-Z0-9_]*)'
        ]
        self.pattern = re.compile(r'\s*(?:{})'.format('|'.join(regex)))
        self.inline = re.compile(r'((?:.|\s)*?);')

    def tokenize(self, program):
        program = re.sub(r'#.*', '', program.rstrip())
        pos = 0
        line = 1
        size = len(program)
        while pos < size:
            match = self.pattern.match(program, pos)
            if not match:
                raise LexerError('Error: invalid token in line {}'.format(line))
            pos = match.end()
            line += program[match.start():match.end()].count('\n')
            for k, v in match.groupdict().items():
                if v:
                    if k == 'int':
                        if v == 'true':
                            v = 1
                        elif v == 'false':
                            v = 0
                        else:
                            v = int(v)
                    elif k == 'char':
                        k = 'int'
                        v = ord(v)
                    elif k == 'id' and v in Lexer.types:
                        k = 'type'
                    # if the token is an operator, a separator or a keyword
                    # k is set to v for easier parsing
                    elif k == 'id' and v in Lexer.control:
                        k = v
                        if v == 'inline':
                            inline = self.inline.match(program, pos)
                            if not inline:
                                raise LexerError('unexpected EOF')
                            pos = inline.end()
                            line += program[inline.start():inline.end()].count('\n')
                            v = re.sub(r'[^+\-><\[\].,]', '', inline[1])
                    elif k == 'op' or k == 'sep':
                        k = v
                    yield Token(k, v, line)
                    break
