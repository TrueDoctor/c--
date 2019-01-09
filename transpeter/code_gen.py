import astnode as ast


def not_yet_implemented():
    raise CodeGenError('functions not yet implemented')


class CodeGenError(Exception):
    pass


class CodeGenerator:
    def __init__(self, ast):
        self.funcs = []
        self.program = ast.Program(ast.name, [])
        for node in ast.instr_list:
            if isinstance(node, ast.Func):
                self.funcs.append(node)
            else:
                self.program.instr_list.append(node)
        self.var_map = [{}]
        self.stack_ptr = 0
        self.base_ptr = [0]

    def generate(self):
        code = '[{}]'.format(self.program.name)  # header comment
        # TODO: check functions for duplicates
        # TODO: check functions for recursion
        if len(self.funcs) > 0:
            not_yet_implemented()
        for node in self.program:
            # generate code
        return code

    def gen_stmnt(self, tree):
        if isinstance(tree, ast.Decl):
            if tree.type == 'void':
                raise CodeGenError(f'line: {tree.line}: variable \'{tree.name}\' declared void')
            if tree.name in self.var_map[-1]:
                raise CodeGenError(f'line: {tree.name}: variable \'{tree.name}\' already declared in same scope')
            code = ''
            if tree.init is not None:
                code += self.eval_expr(tree.init)
            self.var_map[-1][tree.name] = self.stack_ptr
            self.stack_ptr += 1
            return code + '>'
        elif isinstance(tree, ast.Block):
            self.var_map.append({})
            code = ''
            for stmnt in tree.stmnt_list:
                code += self.gen_stmnt(stmnt)
            return code
        elif isinstance(tree, ast.If):
            pass
        elif isinstance(tree, ast.While):
            pass
        elif isinstance(tree, ast.For):
            pass
        elif isinstance(tree, ast.Return):
            not_yet_implemented()
        elif isinstance(tree, ast.FuncCall):
            not_yet_implemented()
        else:
            assert isinstance(tree, ast.Assign)
            name = tree.var
            for scope in reversed(self.var_map):
                if name in scope:
                    addr = scope[name]
                    break
            else:
                raise CodeGenError(f'line: {tree.line}: variable \'{name}\' not declared')
            expr = eval_expr(expression_tree.expr)
            rel_addr = self.stack_ptr - addr
            if tree.op == '=':
                pass
            if tree.op == '+=':
                pass
            if tree.op == '-=':
                pass
            if tree.op == '*=':
                pass
            if tree.op == '/=':
                pass
            if tree.op == '%=':
                pass

    def eval_expr(self, expression_tree):
        if isinstance(expression_tree, ast.BinOp):
            left = eval_expr(expression_tree.left)
            self.stack_ptr += 1
            right = eval_expr(expression_tree.right)
            self.stack_ptr -= 1
            if expression_tree.op == '+':
                return f'{left}>{right}[-<+>]<'
            if expression_tree.op == '-':
                return f'{left}>{right}[-<->]<'
            # TODO: *, /, %
            pass
        elif isinstance(expression_tree, ast.UnOp):
            if expression_tree.op == '+':
                return  eval_expr(expression_tree.right)
            if expression_tree.op == '-':
                self.stack_ptr += 1
                right = eval_expr(expression_tree.right)
                self.stack_ptr -= 1
                return f'[-]>{right}[-<->]<'
        elif isinstance(expression_tree, ast.Var):
            name = expression_tree.name
            for scope in reversed(self.var_map):
                if name in scope:
                    addr = scope[name]
                    break
            else:
                raise CodeGenError(f'line: {expression_tree.line}: variable \'{name}\' not declared)
            rel_addr = self.stack_ptr - addr
            return '{0}[-{1}>+<{0}]{1}>[-<+{0}+{1}>]<'.format('<' * rel_pos, '>' * rel_pos)
        else:  # literal
            assert isinstance(expression_tree, ast.Int)
            return '[-]' + '+' * expression_tree.value
