from dataclasses import dataclass
from enum import Enum, auto
from typing import Any, Optional

from astnode import AstNode, Program


class CompilerError(Exception):
    pass


class TokenType(Enum):
    IDENTIFIER = 'identifier'
    TYPE = 'type'
    OPERATOR = 'operator'
    SEPARATOR = 'separator'
    INT = 'int'
    IF = 'if'
    ELSE = 'else'
    WHILE = 'while'
    REPEAT = 'repeat'
    RETURN = 'return'
    INLINE = 'inline'


@dataclass
class Token:
    line: int
    type: TokenType
    value: Any

    def __repr__(self) -> str:
        return '{}: \'{}\''.format(self.type, self.value)

    def __eq__(self, other) -> bool:
        if isinstance(other, Token):
            return self.type == other.type
        elif isinstance(other, TokenType):
            return self.type == other
        else:
            return self.value == other


class Peekable:
    __slots__ = ['gen', 'peek', 'eof']

    def __init__(self, generator, eof=None):
        self.gen = generator
        self.peek = next(generator, eof)
        self.eof = eof

    def next(self):
        temp = self.peek
        self.peek = next(self.gen, self.eof)
        return temp


@dataclass
class Function:
    args: int
    return_type: str
    code: Optional[str] = None


def print_tree(tree, prefix: str = ''):
    if isinstance(tree, (AstNode, Program)):
        print(tree.__class__.__name__)
        attrs = tree.__print__()
        size = len(attrs)
        for i, (k, v) in enumerate(attrs.items()):
            print(prefix, end='')
            if i == size - 1:
                print('\u2514\u2500', end='')
                print('{}: '.format(k), end='')
                print_tree(v, prefix + '  ')
            else:
                print('\u251c\u2500', end='')
                print('{}: '.format(k), end='')
                print_tree(v, prefix + '\u2502 ')
    elif isinstance(tree, list):
        size = len(tree)
        if size == 0:
            print(None)
        else:
            print()
            for i, node in enumerate(tree):
                print(prefix, end='')
                if i == size - 1:
                    print('\u2514\u2500', end='')
                    print_tree(node, prefix + '  ')
                else:
                    print('\u251c\u2500', end='')
                    print_tree(node, prefix + '\u2502 ')
    else:
        print(tree)
