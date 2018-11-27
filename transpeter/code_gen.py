class CodeGenError(Exception):
    pass


class CodeGenerator:
    def __init__(self, ast):
        self.ast = ast
        self.funcs = []
        for node in ast.nodes:  # maybe change
            if node.name == 'func':
                self.funcs.append(node[1])
        self.var_map = [{}]
        self.stack_ptr = 0
        self.base_ptr = [0]

    def generate(self):
        code = '[{}]\n'.format(self.ast.name)  # header comment
        for node in self.ast:
            pass
        return code

    def _enter_scope(self, *parameters):
        self.var_map.append({})
        for parameter in parameters:
            pass

    def _leave_scope(self):
        pass

    def _declare_var(self, var):
        pass

    def _get_var(self, var):  # pseudocode
        for scope in self.var_map:
            if var in scope:
                return scope[var]
        else:
            raise CodeGenError('line {}: variable \'{}\' not declared'.format(var.line, var))

    def _eval_expr(self):
        pass
