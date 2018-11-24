class CodeGenError(Exception):
    pass


class CodeGenerator:
    def __init__(self, ast):
        self.ast = ast
        self.funcs = []
        self.var_map = [{}]
        self.stack_ptr = 0
        base_ptr = [0]

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

    def _declare_var(self):
        pass

    def _eval_expr(self):
        pass
