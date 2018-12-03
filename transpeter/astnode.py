class AstNode:
    def __init__(self, line):
        self.line = line


class Program(AstNode):
    def __init__(self, name, instructions):
        super().__init__(0)
        self.name = name
        self.instr_list = instructions


class Decl(AstNode):
    def __init__(self, line, var_type, name, init=None):
        super().__init__(line)
        self.type = var_type
        self.name = name
        self.init = init


class Func(AstNode):
    def __init__(self, line, return_type, name, args, block):
        super().__init__(line)
        self.type = return_type
        self.name = name
        self.args = args
        self.block = block


# statements
class Block(AstNode):
    def __init__(self, line, statements):
        super().__init__(line)
        self.stmnt_list = statements


class If(AstNode):
    def __init__(self, line, cond, statement, else_stmnt=None):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement
        self.else_stmnt = else_stmnt


class While(AstNode):
    def __init__(self, line, cond, statement):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement


class For(AstNode):
    def __init__(self, line, cond, statement):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement


class Return(AstNode):
    def __init__(self, line, expr):
        super().__init__(line)
        self.expr = expr


class Inline(AstNode):
    def __init__(self, line, expr):
        super().__init__(line)
        self.expr = expr


class Assign(AstNode):
    def __init__(self, line, op, var, expr):
        super().__init__(line)
        self.op = op
        self.var = var
        self.expr = expr


class FuncCall(AstNode):
    def __init__(self, line, name, args):
        super().__init__(line)
        self.name = name
        self.arg_list = args


# expressions
class BinOp(AstNode):
    def __init__(self, line, op, left, right):
        super().__init__(line)
        self.op = op
        self.left = left
        self.right = right


class UnOp(AstNode):
    def __init__(self, line, op, right):
        super().__init__(line)
        self.op = op
        self.right = right


class Var(AstNode):
    def __init__(self, line, name):
        super().__init__(line)
        self.name = name


class Int(AstNode):
    def __init__(self, line, value):
        super().__init__(line)
        self.value = value
