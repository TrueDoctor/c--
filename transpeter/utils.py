from astnode import AstNode


class CompilerError(Exception):
    pass


class Token:
    __slots__ = ['type', 'value', 'line']

    def __init__(self, token_type, value, line):
        self.type = token_type
        self.value = value
        self.line = line

    def __repr__(self):
        return '{}: \'{}\''.format(self.type, self.value)

    def __eq__(self, other):
        if isinstance(other, Token):
            return self.type == other.type
        else:
            return self.type == other


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


class Function:
    def __init__(self, args, return_type, code=None):
        self.args = args
        self.type = return_type
        self.code = code

    def __repr__(self):
        return f'Function(args={self.args}, type={self.type}, code={self.code})'


def print_tree(tree, prefix=''):
    if isinstance(tree, AstNode):
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
