from utils import AstNode, Peekable


def expect(token, value):
    if token != value:
        raise ParserError('line {}: expected \'{}\', got \'{}\''.format(token.line, value, token.value))


class ParserError(Exception):
    pass


class Parser:
    __slots__ = 'tokens'

    control = ['if', 'else', 'while', 'for', 'return', 'inline']
    EOF = 'eof'

    def __init__(self, tokens):
        self.tokens = Peekable(tokens, 'eof')

    def parse(self, program):
        instr = []
        while self.tokens.peek != Parser.EOF:
            if self.tokens.peek == 'type':  # function or declaration
                var_type, name = self._parse_decl()
                if self.tokens.peek == '=':
                    self.tokens.next()
                    expr = self._parse_expr()
                    next_token = self.tokens.next()
                    expect(next_token, ';')
                    instr.append(AstNode('decl', var_type, name, expr))
                elif self.tokens.peek == ';':
                    self.tokens.next()
                    instr.append(AstNode('decl', var_type, name))
                else:  # function
                    args, block = self._parse_func()
                    instr.append(AstNode('func', var_type, name, args, block))
            else:
                instr.append(self._parse_statement())
        ast = AstNode(program, *instr)
        return ast

    def _parse_func(self):  # returns args, statement (temp?)
        next_token = self.tokens.next()
        expect(next_token, '(')
        args = []
        if self.tokens.peek != ')':
            arg_type = self.tokens.next()
            expect(arg_type, 'type')
            name = self.tokens.next()
            expect(name, 'id')
            args.append(AstNode('decl', arg_type.value, name.value))
        while self.tokens.peek != ')':
            next_token = self.tokens.next()
            expect(next_token, ',')
            arg_type = self.tokens.next()
            expect(arg_type, 'type')
            name = self.tokens.next()
            expect(name, 'id')
            args.append(AstNode('decl', arg_type.value, name.value))
        self.tokens.next()
        block = self._parse_block()
        return AstNode('args', *args), block

    def _parse_block(self):  # blocks
        next_token = self.tokens.next()
        expect(next_token, '{')
        block = []
        while self.tokens.peek != '}':
            if self.tokens.peek == 'type':
                var_type, name = self._parse_decl()
                if self.tokens.peek == '=':
                    self.tokens.next()
                    expr = self._parse_expr()
                    next_token = self.tokens.next()
                    expect(next_token, ';')
                    block.append(AstNode('decl', var_type, name, expr))
                else:
                    next_token = self.tokens.next()
                    expect(next_token, ';')
                    block.append(AstNode('decl', var_type, name))
            else:
                block.append(self._parse_statement())
        self.tokens.next()
        return AstNode('block', *block)

    def _parse_decl(self):
        next_token = self.tokens.next()
        expect(next_token, 'type')
        var_type = next_token.value
        next_token = self.tokens.next()
        expect(next_token, 'id')
        name = next_token.value
        return var_type, name

    def _parse_statement(self):
        if self.tokens.peek == '{':  # block
            return self._parse_block()
        elif self.tokens.peek in ('if', 'for', 'while'):  # for, while, if-else
            ctrl = self.tokens.next()
            next_token = self.tokens.next()
            expect(next_token, '(')
            expr = self._parse_expr()
            next_token = self.tokens.next()
            expect(next_token, ')')
            stmnt = self._parse_statement()
            if ctrl == 'if' and self.tokens.peek == 'else':
                self.tokens.next()
                stmnt2 = self._parse_statement()
                return AstNode('if', expr, stmnt, AstNode('else', stmnt2))
            return AstNode(ctrl.type, expr, stmnt)
        elif self.tokens.peek == 'return':  # return
            self.tokens.next()
            expr = self._parse_expr()
            next_token = self.tokens.next()
            expect(next_token, ';')
            return AstNode('return', expr)
        elif self.tokens.peek == 'inline':
            code = self.tokens.next()
            return AstNode('inline', code.value)
        elif self.tokens.peek == 'id':
            name = self.tokens.next()
            if self.tokens.peek == '(':  # function call
                args = self._parse_func_call()
                next_token = self.tokens.next()
                expect(next_token, ';')
                return AstNode('func_call', name.value, args)
            next_token = self.tokens.next()
            if next_token in ('=', '+=', '-=', '*=', '/=', '%='):  # assignment
                assign_op = next_token.type
                expr = self._parse_expr()
                next_token = self.tokens.next()
                expect(next_token, ';')
                return AstNode(assign_op, name.value, expr)
            else:
                raise ParserError('line {}: expected function call or assignment'.format(next_token.line))
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    def _parse_expr(self):
        expr = self._parse_and()
        while self.tokens.peek == 'or':
            self.tokens.next()
            next_expr = self._parse_and()
            expr = AstNode('or', expr, next_expr)
        return AstNode('expr', expr)

    def _parse_and(self):
        expr = self._parse_equality()
        while self.tokens.peek == 'and':
            self.tokens.next()
            next_expr = self._parse_equality()
            expr = AstNode('and', expr, next_expr)
        return expr

    '''def _parse_compare(self):
        expr = self._parse_additive()
        while self.tokens.peek in ('==', '!=', '<', '>', '<=', '>='):
            op = self.tokens.next()
            next_expr = self._parse_additive()
            expr = AstNode()  # somehow handle 'a < b < c' => 'a < b and b < c'''

    def _parse_equality(self):
        expr = self._parse_relational()
        while self.tokens.peek in ('==', '!='):
            op = self.tokens.next()
            next_expr = self._parse_relational()
            expr = AstNode(op.type, expr, next_expr)
        return expr

    def _parse_relational(self):
        expr = self._parse_additive()
        while self.tokens.peek in ('<', '>', '<=', '>='):
            op = self.tokens.next()
            next_expr = self._parse_additive()
            expr = AstNode(op.type, expr, next_expr)
        return expr

    def _parse_additive(self):
        expr = self._parse_term()
        while self.tokens.peek in ('+', '-'):
            op = self.tokens.next()
            next_expr = self._parse_term()
            expr = AstNode(op.type, expr, next_expr)
        return expr

    def _parse_term(self):
        expr = self._parse_factor()
        while self.tokens.peek in ('*', '/', '%'):
            op = self.tokens.next()
            next_expr = self._parse_factor()
            expr = AstNode(op.type, expr, next_expr)
        return expr

    def _parse_factor(self):
        if self.tokens.peek == 'id':  # function call or variable acess
            next_token = self.tokens.next()
            if self.tokens.peek == '(':
                args = self._parse_func_call()
                return AstNode('func_call', next_token.value, args)
            return AstNode('var', next_token.value)
        elif self.tokens.peek == '(':  # parenthesis
            self.tokens.next()
            expr = self._parse_expr()
            next_token = self.tokens.next()
            expect(next_token, ')')
            return expr
        elif self.tokens.peek in ('not', '-', '+'):  # unary operator
            op = self.tokens.next()
            expr = self._parse_factor()
            return AstNode(op.type, expr)
        elif self.tokens.peek == 'int':  # integer literal
            val = self.tokens.next()
            return AstNode('int', val.value)
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
            next_token = self.tokens.next()
            expect(next_token, ',')
            args.append(self._parse_expr())
        self.tokens.next()
        return AstNode('args', *args)
