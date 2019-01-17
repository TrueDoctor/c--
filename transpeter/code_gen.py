from utils import CompilerError
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
        self.current_func = None
        self.funcs = {}
        self.program = ast.Program(tree.name, [])
        for node in tree.instr_list:
            if isinstance(node, ast.Func):
                if node.name in self.funcs:
                    raise CodeGenError(f'line {node.line}: function \'{node.name}\' defined twice')
                self.funcs[node.name] = node
            else:
                self.program.instr_list.append(node)
        self.var_map = [{}]
        self.stack_ptr = 0
        # vvv might change
        for func in self.funcs:
            if self.check_recursion(func.name, func.block.stmnt_list):
                raise CodeGenError(f'line {func.line}: recursion in function \'func.name\'')

    def check_recursion(self, name, tree):
        for node in tree:
            if isinstance(node, ast.FuncCall):
                if node.name == name:
                    return True
                for func in self.funcs:
                    if func.name == node.name:
                        next_func = func
                        break
                else:
                    raise CodeGenError(f'line {node.line}: function \'{node.name}\' is not defined')
                return self.check_recursion(name, next_func.block.stmnt_list)
            elif isinstance(node, ast.Block):
                for stmnt in node.stmnt_list:
                    if self.check_recursion(name, node.stmnt_list):
                        return True
                return False
            else:
                for attr in vars(node):
                    if isinstance(attr, ast.AstNode):
                        return self.check_recursion(name, node)
        return False

    def generate(self, n=80):
        code = ''
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
                raise CodeGenError('line {tree.line}: function \'{tree.name}\' not defined')
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
                raise CodeGenError('line {expression_tree.line}: function \'{expression_tree.name}\' not defined')
            func = self.funcs[expression_tree.name]
            if func.type == 'void':
                raise CodeGenError('line {expression_tree.line}: function \'{expression_tree.name}\' returns void')
            parameters = len(func.arg_list)
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

    def inline_function(self, node, expr=False):
        if node.name not in self.funcs:
            raise CodeGenError('line {node.line}: function \'{node.name}\' not defined')
        func = self.funcs[node.name]
        if expr and func.type == 'void':
            raise CodeGenError('line {node.line}: function \'{node.name}\' returns void')
        parameters = len(func.arg_list)
        arguments = len(expression_tree.args)
