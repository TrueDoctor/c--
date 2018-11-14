class Token:
     __slots__ = ('type', 'value', 'line')

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
