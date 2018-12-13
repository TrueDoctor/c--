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
            if isinstance(node, ast.Decl):
                pass
            elif isinstance(node, ast.Block):
                pass
            elif isinstance(node, ast.If):
                pass
            elif isinstance(node, ast.While):
                pass
            elif isinstance(node, ast.For):
                pass
            elif isinstance(node, ast.Return):
                not_yet_implemented()
            elif isinstance(node, ast.FuncCall):
                not_yet_implemented()
            else:
                assert isinstance(node, ast.Assign)
                pass
        return code

    def enter_scope(self, *parameters):
        self.var_map.append({})
        for parameter in parameters:
            self.declare_var(parameter)

    def leave_scope(self):
        pass

    def declare_var(self, var):
        pass

    def get_var(self, name):  # pseudocode, should copy to temp location
        for scope in self.var_map:
            if name in scope:
                return scope[name]
        else:
            raise CodeGenError('line {}: variable \'{}\' not declared'.format(name.line, name))

    def eval_expr(self, expression_tree):
        if isinstance(expression_tree, ast.BinOp):
            pass
        elif isinstance(expression_tree, ast.UnOp):
            pass
        elif isinstance(expression_tree, ast.Var):
            pass
        else:  # literal
            pass
