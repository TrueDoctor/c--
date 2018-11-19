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
     
     
class AstNode:
    def __init__(self, name, *nodes):
        self.name = name
        self.nodes = nodes

    def print(self, level=0):
        print(self.name)
        level += 1
        for node in self.nodes:
            print('\t' * level, end='')
            node.print(level)


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
     
     
def print_tree(tree, prefix=""):  # doesn't work
    if isinstance(tree, AstNode):
        print(tree.name)
        size = len(tree.nodes)
        for i in range(size):
            print(prefix, end="")
            if i == size - 1:
                print("\u2514\u2500", end="")
                print_tree(tree.nodes[i], prefix + "  ")
            else:
                print("\u251c\u2500", end="")
                print_tree(tree.nodes[i], prefix + "\u2502 ")
    else:
        print(tree)
