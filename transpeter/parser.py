from utils import Peekable
from astnode import *


class ParserError(Exception):
    pass


class Parser:
    __slots__ = 'tokens'

    control = ['if', 'else', 'while', 'for', 'return', 'inline']
    EOF = 'eof'

    def __init__(self, tokens):
        self.tokens = Peekable(tokens, 'eof')

    def expect(self, value):
        token = self.tokens.next()
        if token != value:
            raise ParserError('line {}: expected \'{}\', got \'{}\''.format(token.line, value, token.value))
        return token

    def parse(self, program):
        instr = []
        while self.tokens.peek != Parser.EOF:
            if self.tokens.peek == 'type':  # function or declaration
                var_type, name = self._parse_decl()
                if self.tokens.peek == '=':  # declaration with initialization
                    self.tokens.next()
                    expr = self._parse_expr()
                    self.expect(';')
                    instr.append(Decl(None, var_type, name, expr))
                elif self.tokens.peek == ';':  # declaration without initialization
                    self.tokens.next()
                    instr.append(Decl(None, var_type, name))
                else:  # function
                    args, block = self._parse_func()
                    instr.append(Func(None, var_type, name, args, block))
            else:  # statement
                instr.append(self._parse_statement())
        ast = Program(program, instr)
        return ast

    def _parse_func(self):  # returns args, statement (temp?)
        self.expect('(')
        args = []
        if self.tokens.peek != ')':
            arg_type = self.expect('type')
            name = self.expect('id')
            args.append(Decl(None, arg_type.value, name.value))
        while self.tokens.peek != ')':
            self.expect(',')
            arg_type = self.expect('type')
            name = self.expect('id')
            args.append(Decl(None, arg_type.value, name.value))
        self.tokens.next()
        block = self._parse_block()
        return args, block

    def _parse_block(self):  # blocks
        line = self.expect('{').line
        block = []
        while self.tokens.peek != '}':
            if self.tokens.peek == 'type':  # declaration
                var_type, name = self._parse_decl()
                if self.tokens.peek == '=':
                    self.tokens.next()
                    expr = self._parse_expr()
                    self.expect(';')
                    block.append(Decl(None, var_type, name, expr))
                else:
                    self.expect(';')
                    block.append(Decl(None, var_type, name))
            else:  # statement
                block.append(self._parse_statement())
        self.tokens.next()
        return Block(line, block)

    def _parse_decl(self):
        var_type = self.expect('type').value
        name = self.expect('id').value
        return var_type, name

    def _parse_statement(self):
        if self.tokens.peek == '{':  # block
            return self._parse_block()
        elif self.tokens.peek in ('if', 'for', 'while'):  # for, while, if-else
            ctrl = self.tokens.next()
            line = ctrl.line
            self.expect('(')
            expr = self._parse_expr()
            self.expect(')')
            stmnt = self._parse_statement()
            if ctrl == 'if':
                if self.tokens.peek == 'else':
                    self.tokens.next()
                    stmnt2 = self._parse_statement()
                    return If(line, expr, stmnt, stmnt2)
                else:
                    return If(line, expr, stmnt)
            elif ctrl == 'while':
                return While(line, expr, stmnt)
            else:
                return For(line, expr, stmnt)
        elif self.tokens.peek == 'return':  # return
            line = self.tokens.next().line
            expr = self._parse_expr()
            self.expect(';')
            return Return(line, expr)
        elif self.tokens.peek == 'inline':  # inline
            inline = self.tokens.next()
            return Inline(inline.line, inline.value)
        elif self.tokens.peek == 'id':
            name = self.tokens.next()
            if self.tokens.peek == '(':  # function call
                args = self._parse_func_call()
                self.expect(';')
                return FuncCall(name.line, name.value, args)
            next_token = self.tokens.next()
            if next_token in ('=', '+=', '-=', '*=', '/=', '%='):  # assignment
                assign_op = next_token.type
                expr = self._parse_expr()
                self.expect(';')
                return Assign(name.line, assign_op, name.value, expr)
            else:
                raise ParserError('line {}: expected function call or assignment'.format(next_token.line))
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    def _parse_expr(self):
        expr = self._parse_and()
        while self.tokens.peek == 'or':
            line = self.tokens.next().line
            next_expr = self._parse_and()
            expr = BinOp(line, 'or', expr, next_expr)
        return expr

    def _parse_and(self):
        expr = self._parse_equality()
        while self.tokens.peek == 'and':
            line = self.tokens.next().line
            next_expr = self._parse_equality()
            expr = BinOp(line, 'and', expr, next_expr)
        return expr

    def _parse_equality(self):
        expr = self._parse_relational()
        while self.tokens.peek in ('==', '!='):
            op = self.tokens.next()
            next_expr = self._parse_relational()
            expr = BinOp(op.line, op.type, expr, next_expr)
        return expr

    def _parse_relational(self):
        expr = self._parse_additive()
        while self.tokens.peek in ('<', '>', '<=', '>='):
            op = self.tokens.next()
            next_expr = self._parse_additive()
            expr = BinOp(op.line, op.type, expr, next_expr)
        return expr

    def _parse_additive(self):
        expr = self._parse_term()
        while self.tokens.peek in ('+', '-'):
            op = self.tokens.next()
            next_expr = self._parse_term()
            expr = BinOp(op.line, op.type, expr, next_expr)
        return expr

    def _parse_term(self):
        expr = self._parse_factor()
        while self.tokens.peek in ('*', '/', '%'):
            op = self.tokens.next()
            next_expr = self._parse_factor()
            expr = BinOp(op.line, op.type, expr, next_expr)
        return expr

    def _parse_factor(self):
        if self.tokens.peek == 'id':  # function call or variable access
            next_token = self.tokens.next()
            if self.tokens.peek == '(':
                args = self._parse_func_call()
                return FuncCall(next_token.line, next_token.value, args)
            return Var(next_token.line, next_token.value)
        elif self.tokens.peek == '(':  # parenthesis
            self.tokens.next()
            expr = self._parse_expr()
            self.expect(')')
            return expr
        elif self.tokens.peek in ('not', '-', '+'):  # unary operator
            op = self.tokens.next()
            expr = self._parse_factor()
            return UnOp(op.line, op.type, expr)
        elif self.tokens.peek == 'int':  # integer literal
            val = self.tokens.next()
            return Int(val.line, val.value)
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    def _parse_func_call(self):  # returns no node, only the arguments
        self.tokens.next()  # is '('
        args = []
        if self.tokens.peek != ')':
            args.append(self._parse_expr())
        while self.tokens.peek != ')':
            self.expect(',')
            args.append(self._parse_expr())
        self.tokens.next()
        return args
