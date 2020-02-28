from utils import TokenType, Peekable, CompilerError
import astnode as ast


class ParserError(CompilerError):
    pass


class Parser:
    __slots__ = 'tokens'

    EOF = 'eof'

    def __init__(self, tokens):
        self.tokens = Peekable(tokens, 'eof')

    def expect(self, token):
        """
        Consumes the next token and raises a `ParserError` if it is not equal to `token`.
        """
        next_token = self.tokens.next()
        if next_token == Parser.EOF:
            raise ParserError(f'unexpected EOF, expected \'{token}\'')
        elif next_token != token:
            raise ParserError(f'line {next_token.line}: expected \'{token}\', got \'{next_token.value}\'')
        return next_token

    def parse(self, program) -> ast.Program:
        instructions = []
        while self.tokens.peek != Parser.EOF:
            if self.tokens.peek == TokenType.TYPE:
                # function or declaration
                var_type, name, line = self.parse_declaration()
                if self.tokens.peek == '=':
                    # declaration with initialization
                    self.tokens.next()
                    expr = self.parse_expression()
                    self.expect(';')
                    instructions.append(ast.Declaration(line, var_type, name, expr))
                elif self.tokens.peek == ';':
                    # declaration without initialization
                    self.tokens.next()
                    instructions.append(ast.Declaration(line, var_type, name))
                else:
                    # function
                    args, block = self.parse_func()
                    instructions.append(ast.Func(line, var_type, name, args, block))
            else:
                # statement
                instructions.append(self.parse_statement())
        return ast.Program(program, instructions)

    # returns args, statement (temp?)
    def parse_func(self):
        self.expect('(')
        args = []
        if self.tokens.peek != ')':
            arg_type = self.expect(TokenType.TYPE)
            name = self.expect(TokenType.IDENTIFIER)
            args.append(ast.Declaration(arg_type.line, arg_type.value, name.value))
        while self.tokens.peek != ')':
            self.expect(',')
            arg_type = self.expect(TokenType.TYPE)
            name = self.expect(TokenType.IDENTIFIER)
            args.append(ast.Declaration(arg_type.line, arg_type.value, name.value))
        self.tokens.next()
        block = self.parse_block()
        return args, block

    def parse_block(self) -> ast.Block:
        """
        Parses a block.
        """
        line = self.expect('{').line
        block = []
        while self.tokens.peek != '}':
            if self.tokens.peek == TokenType.TYPE:
                # declaration
                var_type, name, line = self.parse_declaration()
                if self.tokens.peek == '=':
                    self.tokens.next()
                    expr = self.parse_expression()
                    self.expect(';')
                    block.append(ast.Declaration(line, var_type, name, expr))
                else:
                    self.expect(';')
                    block.append(ast.Declaration(line, var_type, name))
            else:
                # statement
                block.append(self.parse_statement())
        self.tokens.next()
        return ast.Block(line, block)

    def parse_declaration(self):
        """
        Parses a declaration.
        """
        next_token = self.expect(TokenType.TYPE)
        name = self.expect(TokenType.IDENTIFIER).value
        return next_token.value, name, next_token.line

    def parse_statement(self):
        """
        Parses a statement.
        """
        if self.tokens.peek == '{':  # block
            return self.parse_block()
        elif self.tokens.peek in (TokenType.IF, TokenType.REPEAT, TokenType.WHILE):
            # repeat, while, if-else
            ctrl = self.tokens.next()
            line = ctrl.line
            self.expect('(')
            expr = self.parse_expression()
            self.expect(')')
            statement = self.parse_statement()
            if ctrl == TokenType.IF:
                if self.tokens.peek == TokenType.ELSE:
                    self.tokens.next()
                    else_statement = self.parse_statement()
                    return ast.If(line, expr, statement, else_statement)
                else:
                    return ast.If(line, expr, statement)
            elif ctrl == TokenType.WHILE:
                return ast.While(line, expr, statement)
            else:
                assert ctrl == TokenType.REPEAT
                return ast.Repeat(line, expr, statement)
        elif self.tokens.peek == TokenType.RETURN:
            # return
            line = self.tokens.next().line
            expr = self.parse_expression()
            self.expect(';')
            return ast.Return(line, expr)
        elif self.tokens.peek == TokenType.INLINE:
            # inline
            inline = self.tokens.next()
            return ast.Inline(inline.line, inline.value)
        elif self.tokens.peek == TokenType.IDENTIFIER:
            name = self.tokens.next()
            if self.tokens.peek == '(':
                # function call
                args = self.parse_func_call()
                self.expect(';')
                return ast.FuncCall(name.line, name.value, args)
            next_token = self.tokens.next()
            if next_token in ('=', '+=', '-=', '*=', '/=', '%='):
                # assignment
                assign_op = next_token.value
                expr = self.parse_expression()
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

    def parse_expression(self):
        """
        Parses an expression.
        """
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
            expr = ast.BinOp(op.line, op.value, expr, next_expr)
        return expr

    def parse_relational(self):
        expr = self.parse_additive()
        while self.tokens.peek in ('<', '>', '<=', '>='):
            op = self.tokens.next()
            next_expr = self.parse_additive()
            expr = ast.BinOp(op.line, op.value, expr, next_expr)
        return expr

    def parse_additive(self):
        expr = self.parse_term()
        while self.tokens.peek in ('+', '-'):
            op = self.tokens.next()
            next_expr = self.parse_term()
            expr = ast.BinOp(op.line, op.value, expr, next_expr)
        return expr

    def parse_term(self):
        expr = self.parse_factor()
        while self.tokens.peek in ('*', '/', '%'):
            op = self.tokens.next()
            next_expr = self.parse_factor()
            expr = ast.BinOp(op.line, op.value, expr, next_expr)
        return expr

    def parse_factor(self):
        if self.tokens.peek == TokenType.IDENTIFIER:
            # function call or variable access
            next_token = self.tokens.next()
            if self.tokens.peek == '(':
                args = self.parse_func_call()
                return ast.FuncCall(next_token.line, next_token.value, args)
            return ast.Var(next_token.line, next_token.value)
        elif self.tokens.peek == '(':
            # parenthesis
            self.tokens.next()
            expr = self.parse_expression()
            self.expect(')')
            return expr
        elif self.tokens.peek in ('-', '+'):
            # unary operator
            op = self.tokens.next()
            expr = self.parse_factor()
            return ast.UnOp(op.line, op.value, expr)
        elif self.tokens.peek == TokenType.INT:
            # integer literal
            val = self.tokens.next()
            return ast.Int(val.line, val.value)
        elif self.tokens.peek == Parser.EOF:
            raise ParserError('unexpected EOF')
        else:
            raise ParserError('line {}: unexpected token: \'{}\''.format(self.tokens.peek.line, self.tokens.peek.value))

    # returns no node, only the arguments
    def parse_func_call(self):
        self.expect('(')
        args = []
        if self.tokens.peek != ')':
            args.append(self.parse_expression())
        while self.tokens.peek != ')':
            self.expect(',')
            args.append(self.parse_expression())
        self.tokens.next()
        return args
