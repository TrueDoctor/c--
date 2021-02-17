from dataclasses import dataclass
from typing import List, Optional


@dataclass
class AstNode:
    line: int

    def __print__(self) -> dict:
        return {}


@dataclass
class Program:
    name: str
    instructions: List[AstNode]

    def __print__(self) -> dict:
        return {'name': self.name, 'instructions': self.instructions}


@dataclass
class Declaration(AstNode):
    type: str
    name: str
    init: Optional[AstNode] = None

    def __print__(self) -> dict:
        if self.init is None:
            return {'type': self.type, 'name': self.name}
        return {'type': self.type, 'name': self.name, 'init': self.init}


@dataclass
class Func(AstNode):
    return_type: str
    name: str
    args: List[Declaration]
    block: 'Block'

    def __print__(self) -> dict:
        return {'type': self.return_type, 'name': self.name, 'args': self.args, 'statements': self.block.statements}


# statements

@dataclass
class Block(AstNode):
    statements: List[AstNode]

    def __print__(self) -> dict:
        return {'statements': self.statements}


@dataclass
class If(AstNode):
    condition: AstNode
    statement: AstNode
    else_statement: Optional[AstNode] = None

    def __print__(self) -> dict:
        if self.else_statement is None:
            return {'condition': self.condition, 'statement': self.statement}
        return {'condition': self.condition, 'statement': self.statement, 'else statement': self.else_statement}


@dataclass
class While(AstNode):
    condition: AstNode
    statement: AstNode

    def __print__(self) -> dict:
        return {'condition': self.condition, 'statement': self.statement}


@dataclass
class Repeat(AstNode):
    condition: AstNode
    statement: AstNode

    def __print__(self) -> dict:
        return {'condition': self.condition, 'statement': self.statement}


@dataclass
class Return(AstNode):
    expression: AstNode

    def __print__(self) -> dict:
        return {'expression': self.expression}


@dataclass
class Inline(AstNode):
    expression: str

    def __print__(self) -> dict:
        return {'code': self.expression}


@dataclass
class Assign(AstNode):
    op: str
    var: str
    expression: AstNode

    def __print__(self) -> dict:
        return {'operator': self.op, 'variable': self.var, 'expression': self.expression}


@dataclass
class FuncCall(AstNode):
    name: str
    args: List[AstNode]

    def __print__(self) -> dict:
        return {'name': self.name, 'args': self.args}


# expressions

@dataclass
class BinOp(AstNode):
    op: str
    left: AstNode
    right: AstNode

    def __print__(self) -> dict:
        return {'operator': self.op, 'left': self.left, 'right': self.right}


@dataclass
class UnOp(AstNode):
    op: str
    right: AstNode

    def __print__(self) -> dict:
        return {'operator': self.op, 'expression': self.right}


@dataclass
class Var(AstNode):
    name: str

    def __print__(self) -> dict:
        return {'name': self.name}


@dataclass
class Int(AstNode):
    value: int

    def __print__(self) -> dict:
        return {'value': self.value}
