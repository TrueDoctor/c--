class CodeGenerator:
    def __init__(self, ast):
        self.ast = ast
        self.funcs = []
        self.vars = []
        self.stack_ptr = 0
        self.base_ptr = []

        for node in ast.nodes:
            if node.name == 'func':
                self.funcs.append(node)

    def generate(self):
        pass

    def _declare_var(self, var_type):  # currently unnecessary because we only have one type
        # check if var already declared in current scope
        # allocate memory
        pass

    def _eval_exp(self, node):
        pass

    def _create_scope(self, *params):
        self.base_ptr.append(self.stack_ptr)
        for parameter in params:
            self._declare_var(parameter)
