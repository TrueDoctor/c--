from utils import Peekable, CompilerError
import astnode as ast


class ParserError(CompilerError):
    pass


class Parser:
    __slots__ = 'tokens'

    control = ['if', 'else', 'while', 'repeat', 'return', 'inline']
    EOF = 'eof'

    def __init__(self, tokens):
        self.tokens = Peekable(tokens, 'eof')

    def expect(self, value):
        token = self.tokens.next()
        if token == Parser.EOF:
            raise ParserError(f'unexpected EOF, expected \'{value}\'')
        elif token != value:
            raise ParserError(f'line {token.line}: expected \'{value}\', got \'{token.value}\'')
        return token

    def parse(self, program):
        instr = []
        while self.tokens.peek != Parser.EOF:
            if self.tokens.peek in ('type', 'struct'):  # function or declaration
                var_type, name = self.parse_decl()
                if self.tokens.peek == '=':  # declaration with initialization
                    self.tokens.next()
                    expr = self.parse_expr()
                    self.expect(';')
                    instr.append(ast.Decl(var_type.line, var_type, name, expr))
                elif self.tokens.peek == ';':  # declaration without initialization
                    self.tokens.next()
                    instr.append(ast.Decl(var_type.line, var_type, name))
                else:  # function
                    args, block = self.parse_func()
                    instr.append(ast.Func(var_type.line, var_type, name, args, block))
            else:  # statement
                instr.append(self.parse_statement())
        tree = ast.Program(program, instr)
        return tree

    def parse_type(self):
        if self.tokens.peek == 'struct':
            line = self.tokens.next().line
            return ast.Type(line, 'struct ' + self.expect('id').value)
        name = self.expect('type')
        return ast.Type(name.line, name.value)

    def parse_func(self):  # returns args, statement (temp?)
        self.expect('(')
        args = []
        if self.tokens.peek != ')':
            arg_type = self.parse_type()
            name = self.expect('id')
            args.append(ast.Decl(arg_type.line, arg_type.value, name.value))
        while self.tokens.peek != ')':
            self.expect(',')
            arg_type = self.expect('type')
            name = self.expect('id')
            args.append(ast.Decl(arg_type.line, arg_type.value, name.value))
        self.tokens.next()
        block = self.parse_block()
        return args, block

    def parse_block(self):  # blocks
        line = self.expect('{').line
        block = []
        while self.tokens.peek != '}':
            if self.tokens.peek == 'type':  # declaration
                var_type, name, line = self.parse_decl()
                if self.tokens.peek == '=':
                    self.tokens.next()
                    expr = self.parse_expr()
                    self.expect(';')
                    block.append(ast.Decl(line, var_type, name, expr))
                else:
                    self.expect(';')
                    block.append(ast.Decl(line, var_type, name))
            else:  # statement
                block.append(self.parse_statement())
        self.tokens.next()
        return ast.Block(line, block)

    def parse_decl(self):
        var_type = self.parse_type()
        name = self.expect('id').value
        return var_type, name

    def parse_statement(self):
        if self.tokens.peek == '{':  # block
            return self.parse_block()
        elif self.tokens.peek in ('if', 'repeat', 'while'):  # repeat, while, if-else
            ctrl = self.tokens.next()
            line = ctrl.line
            self.expect('(')
            expr = self.parse_expr()
            self.expect(')')
            stmnt = self.parse_statement()
            if ctrl == 'if':
                if self.tokens.peek == 'else':
                    self.tokens.next()
                    stmnt2 = self.parse_statement()
                    return ast.If(line, expr, stmnt, stmnt2)
                else:
                    return ast.If(line, expr, stmnt)
            elif ctrl == 'while':
                return ast.While(line, expr, stmnt)
            else:
                return ast.Repeat(line, expr, stmnt)
        elif self.tokens.peek == 'return':  # return
            line = self.tokens.next().line
            expr = self.parse_expr()
            self.expect(';')
            return ast.Return(line, expr)
        elif self.tokens.peek == 'inline':  # inline
            inline = self.tokens.next()
            return ast.Inline(inline.line, inline.value)
        elif self.tokens.peek == 'id':
            name = self.tokens.next()
            if self.tokens.peek == '(':  # function call
                args = self.parse_func_call()
                self.expect(';')
                return ast.FuncCall(name.line, name.value, args)
            next_token = self.tokens.next()
            if next_token in ('=', '+=', '-=', '*=', '/=', '%='):  # assignment
                assign_op = next_token.type
                expr = self.parse_expr()
                self.expect(';')
                return ast.Assign(name.line, assign_op, name.value, expr)
            elif next_token == Parser.EOF:
                raise ParserError('unexpected EOF')
            else:
                raise ParserError('line {}: expected function call or assignment'.format(next_token.line))
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    def parse_expr(self):
        expr = self.parse_and()
        while self.tokens.peek == 'or':
            line = self.tokens.next().line
            next_expr = self.parse_and()
            expr = ast.BinOp(line, 'or', expr, next_expr)
        return expr

    def parse_and(self):
        expr = self.parse_not()
        while self.tokens.peek == 'and':
            line = self.tokens.next().line
            next_expr = self.parse_not()
            expr = ast.BinOp(line, 'and', expr, next_expr)
        return expr

    def parse_not(self):
        if self.tokens.peek == "not":
            line = self.tokens.next().line
            return ast.UnOp(line, "not", self.parse_equality())
        return self.parse_equality()

    def parse_equality(self):
        expr = self.parse_relational()
        while self.tokens.peek in ('==', '!='):
            op = self.tokens.next()
            next_expr = self.parse_relational()
            expr = ast.BinOp(op.line, op.type, expr, next_expr)
        return expr

    def parse_relational(self):
        expr = self.parse_additive()
        while self.tokens.peek in ('<', '>', '<=', '>='):
            op = self.tokens.next()
            next_expr = self.parse_additive()
            expr = ast.BinOp(op.line, op.type, expr, next_expr)
        return expr

    def parse_additive(self):
        expr = self.parse_term()
        while self.tokens.peek in ('+', '-'):
            op = self.tokens.next()
            next_expr = self.parse_term()
            expr = ast.BinOp(op.line, op.type, expr, next_expr)
        return expr

    def parse_term(self):
        expr = self.parse_factor()
        while self.tokens.peek in ('*', '/', '%'):
            op = self.tokens.next()
            next_expr = self.parse_factor()
            expr = ast.BinOp(op.line, op.type, expr, next_expr)
        return expr

    def parse_factor(self):
        if self.tokens.peek == 'id':  # function call or variable access
            next_token = self.tokens.next()
            if self.tokens.peek == '(':
                args = self.parse_func_call()
                return ast.FuncCall(next_token.line, next_token.value, args)
            return ast.Var(next_token.line, next_token.value)
        elif self.tokens.peek == '(':  # parenthesis
            self.tokens.next()
            expr = self.parse_expr()
            self.expect(')')
            return expr
        elif self.tokens.peek in ('-', '+'):  # unary operator
            op = self.tokens.next()
            expr = self.parse_factor()
            return ast.UnOp(op.line, op.type, expr)
        elif self.tokens.peek == 'int':  # integer literal
            val = self.tokens.next()
            return ast.Int(val.line, val.value)
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    def parse_func_call(self):  # returns no node, only the arguments
        self.expect('(')
        args = []
        if self.tokens.peek != ')':
            args.append(self.parse_expr())
        while self.tokens.peek != ')':
            self.expect(',')
            args.append(self.parse_expr())
        self.tokens.next()
        return args
