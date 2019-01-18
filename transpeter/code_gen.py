from utils import CompilerError, Function
import astnode as ast


def not_yet_implemented():
    raise CodeGenError('functions not yet implemented')

def check_recursion(name, tree):
    for node in tree:
        if isinstance(node, ast.FuncCall):
            if node.name == name:
                return true
            return check_recursion(node)

class CodeGenError(CompilerError):
    pass


class CodeGenerator:
    def __init__(self, tree):
        self.current_funcs = []
        self.funcs = {}
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

    def generate(self, n=80):
        code = ''
        for func in self.funcs.values():
            if func.code is None:
                func.code = self.inline_function(func.node)
        for node in self.program.instr_list:
            code += self.gen_stmnt(node)
        code = '\n'.join([code[i:i+n] for i in range(0, len(code), n)])
        return f'[{self.program.name}]\n' + code

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
            return code
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
        elif isinstance(tree, ast.Return):
            not_yet_implemented()
        elif isinstance(tree, ast.FuncCall):
            if tree.name not in self.func_map:
                raise CodeGenError(f'line {tree.line}: function \'{tree.name}\' not defined')
            func = self.funcs[tree.name]
            pass
        else:
            assert isinstance(tree, ast.Assign)
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
            if expression_tree.name not in self.funcs:
                raise CodeGenError(f'line {expression_tree.line}: function \'{expression_tree.name}\' not defined')
            func = self.funcs[expression_tree.name]
            if func.type == 'void':
                raise CodeGenError('line {expression_tree.line}: function \'{expression_tree.name}\' returns void')
            parameters = len(func.args)
            arguments = len(expression_tree.args)
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
            assert isinstance(expression_tree, ast.Int)
            return '[-]' + '+' * expression_tree.value

    def inline_function(self, node):
        old_var_map = self.var_map
        self.var_map = [{}]
        self.current_funcs.append(node.name)
        code = ''
        for decl in node.args:
            code += self.gen_stmnt(decl)
        for stmnt in func.block.stmnt_list:
            if isinstance(stmnt, ast.FuncCall):
                name = stmnt.name
                if name not in self.funcs:
                    raise CodeGenError(f'line {stmnt.line}: function \'{name}\' does not exist')
                if name in self.current_funcs:
                    raise CodeGenError(f'line {stmnt.line}: function \'{name}\' is recursive')
                if self.funcs[name].code is None:
                    self.funcs[name].code = self.inline_function(self.funcs[name].node)
                code += self.funcs[name]
            else:
                code += self.gen_stmnt(stmnt)
        self.current_funcs.pop()
        self.var_map = old_var_map
        return code
