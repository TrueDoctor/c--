import re
from typing import Optional, Dict

from utils import CompilerError, Function
import astnode as ast


class CodeGenError(CompilerError):
    pass


class CodeGenerator:
    def __init__(self, tree: ast.Program, stdlib: Optional[Dict[str, Function]] = None):
        self.current_funcs = []
        self.functions = {} if stdlib is None else stdlib
        self.function_nodes = {}
        self.program = ast.Program(tree.name, [])
        for node in tree.instructions:
            if isinstance(node, ast.Func):
                name = node.name
                if name in self.function_nodes:
                    raise CodeGenError(f'line {node.line}: function \'{name}\' defined twice')
                self.function_nodes[name] = node
                self.functions[name] = Function(len(node.args), node.return_type)
            else:
                self.program.instructions.append(node)
        self.var_map = [{}]
        self.stack_ptr = 0

    def generate(self, optimize=False, n=80):
        # generate code for functions
        for name, func in self.functions.items():
            if func.code is None:
                self.current_funcs.append(name)
                func.code = self.inline_function(self.function_nodes[name])
                self.current_funcs.pop()
        # generate code for program
        code = ''
        for node in self.program.instructions:
            code += self.gen_statement(node)
        if optimize:
            while re.search(r'\+-|-\+|<>|><', code):
                code = re.sub(r'\+-|-\+|<>|><', '', code)
        code = '\n'.join([code[i:i + n] for i in range(0, len(code), n)])
        return f'[{self.program.name}]\n{code}'

    def gen_statement(self, tree) -> str:
        if isinstance(tree, ast.Declaration):
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
            for statement in tree.statements:
                code += self.gen_statement(statement)
            old_vars = len(self.var_map.pop())
            self.stack_ptr -= old_vars
            return code + '<' * old_vars
        elif isinstance(tree, ast.If):
            if tree.else_statement is None:
                expr = self.eval_expr(tree.condition)
                statement = self.gen_statement(tree.statement)
                return f'{expr}[{statement}[-]]'
            self.stack_ptr += 1
            expr = self.eval_expr(tree.condition)
            statement = self.gen_statement(tree.statement)
            self.stack_ptr -= 1
            else_statement = self.gen_statement(tree.else_statement)
            return f'[-]+>{expr}[{statement}<[-]>[-]]<[{else_statement}[-]]'
        elif isinstance(tree, ast.While):
            expr = self.eval_expr(tree.condition)
            statement = self.gen_statement(tree.statement)
            return f'{expr}[{statement}{expr}]'
        elif isinstance(tree, ast.Repeat):
            expr = self.eval_expr(tree.condition)
            self.stack_ptr += 1
            statement = self.gen_statement(tree.statement)
            self.stack_ptr -= 1
            return f'{expr}[->{statement}<]'
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
            expr = self.eval_expr(tree.expression)
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
            return tree.expression
        else:
            assert isinstance(tree, ast.Return), tree
            if len(self.current_funcs) == 0:
                raise CodeGenError(f'line {tree.line}: return outside of function')
            raise CodeGenError(f'line {tree.line}: invalid position for return')

    def eval_expr(self, expression_tree) -> str:
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
                    addr = scope[name]
                    break
            else:
                raise CodeGenError(f'line {expression_tree.line}: variable \'{name}\' not declared')
            rel_addr = self.stack_ptr - addr
            return '[-]>[-]<{0}[-{1}>+<{0}]{1}>[-<+{0}+{1}>]<'.format('<' * rel_addr, '>' * rel_addr)
        else:
            # literal
            assert isinstance(expression_tree, ast.Int), expression_tree
            return '[-]' + '+' * expression_tree.value

    def inline_function(self, node: ast.Func) -> str:
        old_var_map = self.var_map
        self.var_map = [{}]
        code = ''
        for declaration in node.args:
            code += self.gen_statement(declaration)
        for statement in node.block.statements:
            if isinstance(statement, ast.Return):
                code += self.eval_expr(statement.expression)
                old_vars = len(self.var_map[-1])
                if old_vars > 0:
                    code += '{0}[-]{1}[-{0}+{1}]{0}'.format('<' * old_vars, '>' * old_vars)
                self.stack_ptr -= old_vars
                break
            else:
                code += self.gen_statement(statement)
        else:
            if node.return_type != 'void':
                line = node.block.statements[-1].line if len(node.block.statements) > 0 else node.block.line
                raise CodeGenError(f'line {line}: expected return')
            old_vars = len(self.var_map[-1])
            code += '<' * old_vars
            self.stack_ptr -= old_vars
        self.var_map = old_var_map
        return code

    def function_call(self, node: ast.FuncCall, expr: bool = False) -> str:
        if node.name in self.current_funcs:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' is recursive')
        self.current_funcs.append(node.name)
        if node.name not in self.functions:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' not defined')
        func = self.functions[node.name]
        if expr and func.return_type == 'void':
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' returns void')
        args = len(node.args)
        params = func.args
        if args != params:
            raise CodeGenError(f'line {node.line}: function \'{node.name}\' expects {params} arguments, got {args}')
        code = ''
        for arg in node.args:
            code += self.eval_expr(arg) + '>'
            self.stack_ptr += 1
        code += '<' * args
        self.stack_ptr -= args
        if func.code is None:
            func.code = self.inline_function(self.function_nodes[node.name])
        self.current_funcs.pop()
        return code + func.code
