class AstNode:
    def __init__(self, line):
        self.line = line

    def __print__(self):
        return {}


class Program(AstNode):
    def __init__(self, name, instructions):
        super().__init__(0)
        self.name = name
        self.instr_list = instructions

    def __print__(self):
        return {'name': self.name, 'instructions': self.instr_list}


class Decl(AstNode):
    def __init__(self, line, var_type, name, init=None):
        super().__init__(line)
        self.type = var_type
        self.name = name
        self.init = init

    def __print__(self):
        if self.init is None:
            return {'type': self.type, 'name': self.name}
        return {'type': self.type, 'name': self.name, 'init': self.init}


class Func(AstNode):
    def __init__(self, line, return_type, name, args, block):
        super().__init__(line)
        self.type = return_type
        self.name = name
        self.args = args
        self.block = block

    def __print__(self):
        return {'type': self.type, 'name': self.name, 'args': self.args, 'statements': block.stmnt_list}


# statements
class Block(AstNode):
    def __init__(self, line, statements):
        super().__init__(line)
        self.stmnt_list = statements

    def __print__(self):
        return {'statements': self.stmnt_list}


class If(AstNode):
    def __init__(self, line, cond, statement, else_stmnt=None):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement
        self.else_stmnt = else_stmnt

    def __print__(self):
        if self.else_stmnt is None:
            return {'condition': self.cond, 'statement': self.stmnt}
        return {'condition': self.cond, 'statement': self.stmnt, 'else statement': self.else_stmnt}


class While(AstNode):
    def __init__(self, line, cond, statement):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement

    def __print__(self):
        return {'condition': self.cond, 'statement': self.stmnt}


class Repeat(AstNode):
    def __init__(self, line, cond, statement):
        super().__init__(line)
        self.cond = cond
        self.stmnt = statement

    def __print__(self):
        return {'condition': self.cond, 'statement': self.stmnt}


class Return(AstNode):
    def __init__(self, line, expr):
        super().__init__(line)
        self.expr = expr

    def __print__(self):
        return {'expression': self.expr}


class Inline(AstNode):
    def __init__(self, line, expr):
        super().__init__(line)
        self.expr = expr

    def __print__(self):
        return {'code': self.expr}


class Assign(AstNode):
    def __init__(self, line, op, var, expr):
        super().__init__(line)
        self.op = op
        self.var = var
        self.expr = expr

    def __print__(self):
        return {'operator': self.op, 'variable': self.var, 'expression': self.expr}


class FuncCall(AstNode):
    def __init__(self, line, name, args):
        super().__init__(line)
        self.name = name
        self.args = args

    def __print__(self):
        return {'name': self.name, 'args': self.args}


# expressions
class BinOp(AstNode):
    def __init__(self, line, op, left, right):
        super().__init__(line)
        self.op = op
        self.left = left
        self.right = right

    def __print__(self):
        return {'operator': self.op, 'left': self.left, 'right': self.right}


class UnOp(AstNode):
    def __init__(self, line, op, right):
        super().__init__(line)
        self.op = op
        self.right = right

    def __print__(self):
        return {'operator': self.op, 'expression': self.right}


class Var(AstNode):
    def __init__(self, line, name):
        super().__init__(line)
        self.name = name

    def __print__(self):
        return {'name': self.name}


class Int(AstNode):
    def __init__(self, line, value):
        super().__init__(line)
        self.value = value

    def __print__(self):
        return {'value': self.value}
