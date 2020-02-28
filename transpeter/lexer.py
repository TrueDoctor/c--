import re
from typing import Iterable

from utils import Token, TokenType, CompilerError


class LexerError(CompilerError):
    pass


class Lexer:
    __slots__ = ['pattern', 'inline']

    ops = ['+=', '+', '-=', '-', '*=', '*', '/=', '/', '%=', '%', '==', '!=', '>=', '<=', '>', '<', 'or', 'and', 'not']
    separators = ['=', '{', '}', '(', ')', ';', ',']
    types = ['void', 'int']
    keywords = ['if', 'else', 'while', 'repeat', 'return', 'inline']
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

    def tokenize(self, program: str) -> Iterable[Token]:
        program = re.sub(r'#.*', '', program).rstrip()
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
                    token_type = TokenType.IDENTIFIER
                    if k == 'int':
                        token_type = TokenType.INT
                        if v == 'true':
                            v = 1
                        elif v == 'false':
                            v = 0
                        else:
                            v = int(v)
                    elif k == 'char':
                        token_type = TokenType.INT
                        v = ord(v)
                    elif k == 'id' and v in Lexer.types:
                        token_type = TokenType.TYPE
                    # v is set to k for keywords
                    elif k == 'id' and v in Lexer.keywords:
                        token_type = TokenType(v)
                        if v == 'inline':
                            inline = self.inline.match(program, pos)
                            if not inline:
                                raise LexerError('unexpected EOF')
                            pos = inline.end()
                            line += program[inline.start():inline.end()].count('\n')
                            v = re.sub(r'[^+\-><\[\].,]', '', inline[1])
                    elif k == 'op':
                        token_type = TokenType.OPERATOR
                    elif k == 'sep':
                        token_type = TokenType.SEPARATOR
                    yield Token(line, token_type, v)
                    break
