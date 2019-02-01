import re

from utils import CompilerError, Function, Struct, Variable
import astnode as ast


class CodeGenError(CompilerError):
    pass


class CodeGenerator:
    def __init__(self, tree, stdlib=None):
        self.current_funcs = []
        self.funcs = {} if stdlib is None else stdlib
        self.structs = {}
        self.func_nodes = {}
        self.program = ast.Program(tree.name, [])
        for node in tree.instr_list:
            if isinstance(node, ast.Function):
                if node.name in self.func_nodes:
                    raise CodeGenError(f'line {node.line}: function \'{node.name}\' defined twice')
                self.func_nodes[node.name] = node
                self.funcs[node.name] = Function(node)
            elif isinstance(node, ast.Struct):
                if node.name in self.structs:
                    raise CodeGenError(f'line {node.line}: struct \'{node.name}\' defined twice')
                self.funcs[node.name] = Struct(node)  # TODO: change
            else:
                self.program.instr_list.append(node)
        self.var_map = [{}]
        self.stack_ptr = 0

    def generate(self, optimize=False, n=80):
        # generate code for functions
        for name, node in self.func_nodes.items():
            if self.funcs[name].code is None:
                self.current_funcs.append(name)
                self.funcs[name].code = self.inline_function(node)
                self.current_funcs.pop()
        # generate code for program
        code = ''
        for node in self.program.instr_list:
            code += self.gen_stmnt(node)
        if optimize:
            while re.search(r'\+-|-\+|<>|><', code):
                code = re.sub(r'\+-|-\+|<>|><', '', code)
        code = '\n'.join([code[i:i + n] for i in range(0, len(code), n)])
        return f'[{self.program.name}]\n{code}'

    def gen_stmnt(self, tree):
        if isinstance(tree, ast.Declaration):
            if tree.type.name == 'void':
                raise CodeGenError(f'line {tree.line}: variable \'{tree.name}\' declared void')
            if tree.name in self.var_map[-1]:
                raise CodeGenError(f'line {tree.line}: variable \'{tree.name}\' already declared in same scope')
            code = ''
            if tree.init is not None:
                code += self.eval_expr(tree.init)
            self.var_map[-1][tree.name] = Variable(tree.type, self.stack_ptr)  # TODO
            self.stack_ptr += 1  # TODO: change for structs
            return code + '>'
        elif isinstance(tree, ast.Block):
            self.var_map.append({})
            code = ''
            for stmnt in tree.stmnt_list:
                code += self.gen_stmnt(stmnt)
            old_vars = len(self.var_map.pop())
            self.stack_ptr -= old_vars
            return code + '<' * old_vars
        elif isinstance(tree, ast.If):
            expr = self.eval_expr(tree.cond)
            stmnt = self.gen_stmnt(tree.stmnt)
            if tree.else_stmnt is not None:
                else_stmnt = self.gen_stmnt(tree.else_stmnt)
                return f'{expr}>[-]+<[{stmnt}>[-]<[-]]>[{else_stmnt}[-]]<'
            return f'{expr}[{stmnt}[-]]'
        elif isinstance(tree, ast.While):
            expr = self.eval_expr(tree.cond)
            stmnt = self.gen_stmnt(tree.stmnt)
            return f'{expr}[{stmnt}{expr}]'
        elif isinstance(tree, ast.Repeat):
            expr = self.eval_expr(tree.cond)
            self.stack_ptr += 1
            stmnt = self.gen_stmnt(tree.stmnt)
            self.stack_ptr -= 1
            return f'{expr}[->{stmnt}<]'
        elif isinstance(tree, ast.FuncCall):
            return self.function_call(tree)
        elif isinstance(tree, ast.Assign):
            name = tree.var
            for scope in reversed(self.var_map):
                if name in scope:
                    addr = scope[name].addr  # TODO
                    break
            else:
                raise CodeGenError(f'line {tree.line}: variable \'{name}\' not declared')
            expr = self.eval_expr(tree.expr)
            rel_addr = self.stack_ptr - addr
            if tree.op == '=':
                return '{0}[-]{1}{2}[-{0}+{1}]'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '+=':
                return '{2}[-{0}+{1}]'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '-=':
                return '{2}[-{0}-{1}]'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '*=':
                return '{2}>[-]>[-]<<{0}[-{1}>+<{0}]{1}[->[->+<<{0}+{1}>]>[-<+>]<<]'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '/=':
                return '{2}>[-]>[-]>[-]>[-]<<<<{0}[-{1}>>+<<{0}]{1}>>[-<+<-[->>>+>+<<<<]>>>[-<<<+>>>]+>[<->[-]]<[<<<{0}+{1}>[-<+>]>>[-]]<]<<'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '%=':
                return '{2}>[-]>[-]>[-]<<<{0}[-{1}->+<[->>+>+<<<]>>[-<<+>>]+>[<->[-]]<[<[-<+>]>[-]]<<{0}]{1}>[-<{0}+{1}>]<'.format('<' * rel_addr, '>' * rel_addr, expr)
        elif isinstance(tree, ast.Inline):
            return tree.expr
        else:
            assert isinstance(tree, ast.Return), tree
            if len(self.current_funcs) == 0:
                raise CodeGenError(f'line {tree.line}: return outside of function')
            raise CodeGenError(f'line {tree.line}: invalid position for return')

    def eval_expr(self, expression_tree):
        if isinstance(expression_tree, ast.BinOp):
            left = self.eval_expr(expression_tree.left)
            self.stack_ptr += 1
            right = self.eval_expr(expression_tree.right)
            self.stack_ptr -= 1
            if expression_tree.op == '+':
                return f'{left}>{right}[-<+>]<'
            if expression_tree.op == '-':
                return f'{left}>{right}[-<->]<'
            if expression_tree.op == '*':
                return f'{left}>{right}>[-]>[-]<<<[->>+<<]>[->[->+<<<+>>]>[-<+>]<<]<'
            if expression_tree.op == '/':
                return f'{left}>{right}>[-]>[-]>[-]>[-]<<<<<[->->+<[->>>+>+<<<<]>>>[-<<<+>>>]+>[<->[-]]<[<+<[-<+>]>>[-]]<<<<]>>>[-<<<+>>>]<<<'
            if expression_tree.op == '%':
                return f'{left}>{right}>[-]>[-]>[-]<<<<[->->+<[->>+>+<<<]>>[-<<+>>]+>[<->[-]]<[<[-<+>]>[-]]<<<]>>[-<<+>>]<<'
            if expression_tree.op == '<':
                return f'{left}>{right}>[-]>[-]<<<[->[->+>+<<]>[-<+>]>[<<->>[-]]<<<]>[<+>[-]]<'
            if expression_tree.op == '>':
                return f'{left}>{right}>[-]>[-]<<[-<[->>+>+<<<]>>[-<<+>>]>[<<<->>>[-]]<<]<[>+<[-]]>[-<+>]<'
            if expression_tree.op == '<=':
                return f'{left}>{right}>[-]>[-]<<[-<[->>+>+<<<]>>[-<<+>>]>[<<<->>>[-]]<<]<[>+<[-]]+>[-<->]<'
            if expression_tree.op == '>=':
                return f'{left}>{right}>[-]>[-]<<<[->[->+>+<<]>[-<+>]>[<<->>[-]]<<<]+>[<->[-]]<'
            if expression_tree.op == "==":
                return f'{left}>{right}<[->-<]+>[<->[-]]<'
            if expression_tree.op == '!=':
                return f'{left}>{right}<[->-<]>[<+>[-]]<'
            if expression_tree.op == 'or':
                return f'{left}>{right}>[-]<<[>>+<<[-]]>[>[-]+<[-]]>[-<<+>>]<<'
            if expression_tree.op == 'and':
                return f'{left}>{right}>[-]<[<[>>+<<[-]]>[-]]<[-]>>[-<<+>>]<<'
        elif isinstance(expression_tree, ast.UnOp):
            if expression_tree.op == '+':
                return self.eval_expr(expression_tree.right)
            if expression_tree.op == '-':
                self.stack_ptr += 1
                right = self.eval_expr(expression_tree.right)
                self.stack_ptr -= 1
                return f'[-]>{right}[-<->]<'
            if expression_tree.op == 'not':
                self.stack_ptr += 1
                expr = self.eval_expr(expression_tree.right)
                self.stack_ptr -= 1
                return f'[-]+>{expr}[<->[-]]<'
        elif isinstance(expression_tree, ast.FuncCall):
            return self.function_call(expression_tree, expr=True)
        elif isinstance(expression_tree, ast.Var):
            name = expression_tree.name
            for scope in reversed(self.var_map):
                if name in scope:
                    addr = scope[name].addr  # TODO
                    break
            else:
                raise CodeGenError(f'line {expression_tree.line}: variable \'{name}\' not declared')
            rel_addr = self.stack_ptr - addr
            return '[-]>[-]<{0}[-{1}>+<{0}]{1}>[-<+{0}+{1}>]<'.format('<' * rel_addr, '>' * rel_addr)
        else:  # literal
            assert isinstance(expression_tree, ast.Int), expression_tree
            return '[-]' + '+' * expression_tree.value

    def inline_function(self, node):
        old_var_map = self.var_map
        self.var_map = [{}]
        code = ''
        for decl in node.args:
            code += self.gen_stmnt(decl)
        for stmnt in node.block.stmnt_list:
            if isinstance(stmnt, ast.Return):
                code += self.eval_expr(stmnt.expr)
                old_vars = len(self.var_map[-1])
                if old_vars > 0:
                    code += '{0}[-]{1}[-{0}+{1}]{0}'.format('<' * old_vars, '>' * old_vars)
                self.stack_ptr -= old_vars
                break
            else:
                code += self.gen_stmnt(stmnt)
        else:
            if node.type.name != 'void':
                line = node.block.stmnt_list[-1].line if len(node.block.stmnt_list) > 0 else node.block.line
                raise CodeGenError(f'line {line}: expected return')
            old_vars = len(self.var_map[-1])
            code += '<' * old_vars
            self.stack_ptr -= old_vars
        self.var_map = old_var_map
        return code

    def function_call(self, node, expr=False):
        if node.name in self.current_funcs:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' is recursive')
        self.current_funcs.append(node.name)
        if node.name not in self.funcs:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' not defined')
        func = self.funcs[node.name]
        if expr and func.type.name == 'void':
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' returns void')
        args = len(node.args)
        params = len(func.args)
        if args != params:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' expects {params} arguments, got {args}')
        code = ''
        for arg in node.args:
            code += self.eval_expr(arg) + '>'
            self.stack_ptr += 1
        code += '<' * args
        self.stack_ptr -= args
        if func.code is None:
            func.code = self.inline_function(self.func_nodes[node.name])
        self.current_funcs.pop()
        return code + func.code
