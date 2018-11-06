import re
import sys


class Lexer:
    __slots__ = ("program", "pattern")

    def __init__(self, program):
        self.program = program
        regex = ["(?P<sep>\(|\)|{|}|;|,|\s)", "(?P<op>\+|-|\*|/|%)", "(?P<int>[0-9]+)", "(?P<id>[a-zA-Z_][a-zA-Z0-9_]*)"]
        self.pattern = re.compile("|".join(regex))

    def tokenize(self):
        tokens = []
        line = 0
        while True:
            m = self.pattern.match("self.program")
            if not m:
                sys.exit("Error: invalid token in line {}".format(line))

        return tokens
