import re

from utils import CompilerError, Function
from builtin import precompile
import astnode as ast


class CodeGenError(CompilerError):
    pass


class CodeGenerator:
    def __init__(self, tree):
        self.current_funcs = []
        self.funcs = precompile()
        self.program = ast.Program(tree.name, [])
        for node in tree.instr_list:
            if isinstance(node, ast.Func):
                if node.name in self.funcs:
                    raise CodeGenError(f'line {node.line}: function \'{node.name}\' defined twice')
                self.funcs[node.name] = Function(node)
            else:
                self.program.instr_list.append(node)
        self.var_map = [{}]
        self.stack_ptr = 0

    def generate(self, optimize=False, n=80):
        for func in self.funcs.values():
            if func.code is None:
                self.current_funcs.append(func.node.name)
                func.code = self.inline_function(func.node)
                self.current_funcs.pop()
        code = ''
        for node in self.program.instr_list:
            code += self.gen_stmnt(node)
        code = '\n'.join([code[i:i+n] for i in range(0, len(code), n)])
        if optimize:
            while re.search(r'\+-|-\+|<>|><', code):
                code = re.sub(r'\+-|-\+|<>|><', '', code)
        return f'[{self.program.name}]\n{code}'

    def gen_stmnt(self, tree):
        if isinstance(tree, ast.Decl):
            if tree.type == 'void':
                raise CodeGenError(f'line {tree.line}: variable \'{tree.name}\' declared void')
            if tree.name in self.var_map[-1]:
                raise CodeGenError(f'line {tree.line}: variable \'{tree.name}\' already declared in same scope')
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
                    addr = scope[name]
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
                return '{2}>[-]>[-]<<{0}[-{1}>>+<<{0}]{1}>>[<<[{0}+{1}>+<-]>[<+>-]>-]<<'.format('<' * rel_addr, '>' * rel_addr, expr)
            if tree.op == '/=':
                pass
            if tree.op == '%=':
                pass
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
                return f'{left}>{right}>[-]>[-]<<<[->>>+<<<]>>>[<<[<+>>+<-]>[<+>-]>-]<<<'
            if expression_tree.op == '/':
                pass
            if expression_tree.op == '%':
                pass
            if expression_tree.op == '<':
                pass
            if expression_tree.op == '>':
                pass
            if expression_tree.op == '<=':
                pass
            if expression_tree.op == '>=':
                pass
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
                    addr = scope[name]
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
                if len(node.args) > 0:
                    code += '{0}[-]{1}[-{0}+{1}]{0}'.format('<' * old_vars, '>' * old_vars)
                self.stack_ptr -= old_vars
                break
            else:
                code += self.gen_stmnt(stmnt)
        else:
            if node.type != 'void':
                raise CodeGenError(f'line {node.block.stmnt_list[-1].line if len(node.block.stmnt_list) > 0 else node.block.line}: expected return')
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
        if expr and func.node.type == 'void':
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' returns void')
        arguments = len(node.args)
        parameters = len(func.node.args)
        if arguments != parameters:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' expects {parameters} arguments, got {arguments}')
        code = ''
        for arg in node.args:
            code += self.eval_expr(arg) + '>'
            self.stack_ptr += 1
        code += '<' * len(node.args)
        self.stack_ptr -= len(node.args)
        if func.code is None:
            func.code = self.inline_function(func.node)
        self.current_funcs.pop()
        return code + func.code
