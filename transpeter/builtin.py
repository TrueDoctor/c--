import astnode as ast

LINE = 'built-in'

PUTCHAR = ast.Func(LINE, 'void', 'putchar', [ast.Decl(LINE, 'int', 'arg')], ast.Block(LINE, [ast.Inline(LINE, '<.>')]))
GETCHAR = ast.Func(LINE, 'int', 'getchar', [], ast.Block(LINE, [ast.Return(LINE, ast.Inline(LINE, ','))]))
