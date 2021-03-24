//! The code generation.
//!
//! It assumes that the cells of the brainfuck implementation is unsigned.

mod optimizations;

use std::collections::HashMap;
use std::fmt::Write;
use std::mem;

use crate::ast;
use crate::util::{compiler_error, CompilerResult};
use optimizations::optimize_code;

/// A compiled program.
#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub functions: HashMap<String, Function>,
    pub code: String,
}

/// A compiled function.
#[derive(Debug)]
pub struct Function {
    pub void: bool,
    pub arity: usize,
    pub code: String,
}

type VarMap = Vec<HashMap<String, usize>>;

struct CodeGen {
    variables: VarMap,
    functions: HashMap<String, Function>,
    stack_ptr: usize,
    code: String,
    current_function: Option<String>,
}

impl CodeGen {
    fn new() -> Self {
        Self {
            variables: Self::new_var_map(),
            functions: HashMap::new(),
            stack_ptr: 0,
            code: String::new(),
            current_function: None,
        }
    }

    // variables

    fn new_var_map() -> VarMap {
        vec![HashMap::new()]
    }

    fn declared(&self, var: &str) -> bool {
        self.variables
            .iter()
            .last()
            .map(|scope| scope.contains_key(var))
            .expect("no scope available")
    }

    fn define_var(&mut self, name: String) {
        self.variables
            .last_mut()
            .expect("no scope available")
            .insert(name, self.stack_ptr);
        self.stack_ptr += 1;
        self.code.push('>');
    }

    fn lookup_var(&mut self, name: &str) -> Option<usize> {
        self.variables
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .copied()
    }

    fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        let old_vars = self.variables.pop().expect("no scope available").len();
        self.stack_ptr -= old_vars;
        for _ in 0..old_vars {
            self.code.push('<');
        }
    }

    // generate

    fn generate_function(&mut self, func: ast::ItemFunction) -> CompilerResult<()> {
        let mut func_name = func.name.value;
        if self.functions.contains_key(&func_name) {
            return compiler_error(
                func.name.pos,
                format!("function `{}` is defined multiple times", func_name),
            );
        }
        let old_variables = mem::replace(&mut self.variables, Self::new_var_map());
        let old_code = mem::replace(&mut self.code, String::new());

        // check parameters
        let arity = func.parameters.len();
        for param in func.parameters {
            if param.type_.value == "void" {
                return compiler_error(
                    param.type_.pos,
                    format!("parameter `{}` has type `void`", param.name.value),
                );
            } else if self.declared(&param.name.value) {
                return compiler_error(
                    param.type_.pos,
                    format!(
                        "parameter `{}` is declared multiple times",
                        param.name.value
                    ),
                );
            }
            self.define_var(param.name.value);
        }

        // generate body
        self.current_function = Some(func_name);
        let mut has_return = false;
        let void = func.return_type.value == "void";
        for stmt in func.statements {
            if let ast::Statement::Return { expr, pos } = &stmt {
                if void {
                    return compiler_error(
                        *pos,
                        "unexpected `return` statement in function returning `void`",
                    );
                }
                self.generate_expr(expr)?;
                let old_vars = self.variables.last().expect("no scope available").len();
                if old_vars > 0 {
                    // move return value
                    let left = "<".repeat(old_vars);
                    let right = ">".repeat(old_vars);
                    self.code.push_str(&left);
                    self.code.push_str("[-]");
                    self.code.push_str(&right);
                    self.code.push_str("[-");
                    self.code.push_str(&left);
                    self.code.push('+');
                    self.code.push_str(&right);
                    self.code.push(']');
                }
                has_return = true;
                break;
            }
            self.generate_statement(stmt)?;
        }
        func_name = self
            .current_function
            .take()
            .expect("no current function available");
        if !(has_return || void) {
            return compiler_error(
                func.name.pos,
                format!("function `{}` has no `return` statement", func_name),
            );
        }

        self.exit_scope();
        let code = mem::replace(&mut self.code, old_code);
        self.variables = old_variables;
        self.functions.insert(func_name, Function { void, arity, code });
        Ok(())
    }

    fn generate_statement(&mut self, stmt: ast::Statement) -> CompilerResult<()> {
        use ast::Statement::*;

        match stmt {
            Declaration(decl) => {
                if decl.type_.value == "void" {
                    return compiler_error(
                        decl.type_.pos,
                        format!("variable `{}` has type `void`", decl.name.value),
                    );
                } else if self.declared(&decl.name.value) {
                    return compiler_error(
                        decl.type_.pos,
                        format!("parameter `{}` is declared multiple times", decl.name.value),
                    );
                }
                if let Some(expr) = decl.init {
                    self.generate_expr(&expr)?;
                }
                self.define_var(decl.name.value);
            }
            Block(statements) => {
                self.enter_scope();
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
                self.exit_scope();
            }
            If {
                condition,
                then_statement,
                else_statement,
                ..
            } => match else_statement {
                Some(else_statement) => {
                    // "[-]+>{condition}[{statement}<->[-]]<[{else_statement}[-]]"
                    self.code.push_str("[-]+>");
                    self.stack_ptr += 1;
                    self.generate_expr(&condition)?;
                    self.code.push('[');
                    self.generate_statement(*then_statement)?;
                    self.stack_ptr -= 1;
                    self.code.push_str("<->[-]]<[");
                    self.generate_statement(*else_statement)?;
                    self.code.push_str("[-]]");
                }
                None => {
                    // "{condition}[{statement}[-]]"
                    self.generate_expr(&condition)?;
                    self.code.push('[');
                    self.generate_statement(*then_statement)?;
                    self.code.push_str("[-]]");
                }
            },
            While { condition, statement, .. } => {
                // "{condition}[{statement}{condition}]"
                let old_code = mem::replace(&mut self.code, String::new()); // temporarily replace self.code
                self.generate_expr(&condition)?;
                let cond = mem::replace(&mut self.code, old_code);
                self.code.push_str(&cond);
                self.code.push('[');
                self.generate_statement(*statement)?;
                self.code.push_str(&cond);
                self.code.push(']');
            }
            Repeat { expr, statement, .. } => {
                // "{expr}[>{statement}<-]"
                self.generate_expr(&expr)?;
                self.code.push_str("[>");
                self.stack_ptr += 1;
                self.generate_statement(*statement)?;
                self.stack_ptr -= 1;
                self.code.push_str("<-]");
            }
            Return { pos, .. } => return compiler_error(pos, "invalid `return` statement"),
            Inline { code, .. } => self.code.push_str(std::str::from_utf8(&code).unwrap()),
            Assign { name, op, expr } => {
                use ast::AssignOpKind::*;

                let rel_addr = match self.lookup_var(&name.value) {
                    Some(addr) => self.stack_ptr - addr,
                    None => {
                        return compiler_error(
                            name.pos,
                            format!("undeclared variable `{}`", name.value),
                        )
                    }
                };
                let left = "<".repeat(rel_addr);
                let right = ">".repeat(rel_addr);
                self.generate_expr(&expr)?;
                match op.kind {
                    Eq => {
                        write!(self.code, "{left}[-]{right}[-{left}+{right}]", left = left, right = right).unwrap();
                    }
                    PlusEq => {
                        write!(self.code, "[-{left}+{right}]", left = left, right = right).unwrap();
                    }
                    MinusEq => {
                        write!(self.code, "[-{left}-{right}]", left = left, right = right).unwrap();
                    }
                    StarEq => {
                        write!(self.code, ">[-]>[-]<<{left}[-{right}>+<{left}]{right}[->[->+<<{left}+{right}>]>[-<+>]<<]", left = left, right = right).unwrap();
                    }
                    SlashEq => {
                        write!(self.code, ">[-]+>[-]>[-]>[-]<<<<{left}[-{right}-[>+>>]>[[-<+>]+>+>>]<<<<{left}]{right}>>[-<<{left}+{right}>>]<<", left = left, right = right).unwrap();
                    }
                    PercentEq => {
                        write!(self.code, ">[-]+>[-]>[-]>[-]<<<<{left}[-{right}-[>+>>]>[[-<+>]+> >>]<<<<{left}]{right}>-[-<{left}+{right}>]<", left = left, right = right).unwrap();
                    }
                }
            }
            Call { name, args } => self.generate_call(&name, &args, false)?,
        }
        Ok(())
    }

    fn generate_expr(&mut self, expr: &ast::Expr) -> CompilerResult<()> {
        use ast::Expr::*;

        match expr {
            Binary { left, op, right } => {
                use ast::BinaryOpKind::*;

                self.generate_expr(left)?;
                self.code.push('>');
                self.stack_ptr += 1;
                self.generate_expr(right)?;
                self.stack_ptr -= 1;
                self.code.push_str(match op.kind {
                    Plus => "[-<+>]",
                    Minus => "[-<->]",
                    Star => ">[-]>[-]<<<[->>+<<]>[->[->+<<<+>>]>[-<+>]<<]",
                    Slash => ">[-]+>[-]>[-]>[-]<<<<<[->-[>+>>]>[[-<+>]+>+>>]<<<<<]>>>[-<<<+>>>]<<",
                    Percent => ">[-]+>[-]>[-]>[-]<<<<<[->-[>+>>]>[[-<+>]+> >>]<<<<<]>>-[-<<+>>]<",
                    EqEq => "<[->-<]+>[<->[-]]",
                    NotEq => "<[->-<]>[<+>[-]]",
                    Greater => ">[-]>[-]<<[-<[->>+>+<<<]>>[-<<+>>]>[<<<->>>[-]]<<]<[>+<[-]]>[-<+>]",
                    GreaterEq => ">[-]>[-]<<<[->[->+>+<<]>[-<+>]>[<<->>[-]]<<<]+>[<->[-]]",
                    Less => ">[-]>[-]<<<[->[->+>+<<]>[-<+>]>[<<->>[-]]<<<]>[<+>[-]]",
                    LessEq => ">[-]>[-]<<[-<[->>+>+<<<]>>[-<<+>>]>[<<<->>>[-]]<<]<[>+<[-]]+>[-<->]",
                    And => ">[-]<[<[>>+<<[-]]>[-]]<[-]>>[-<<+>>]<",
                    Or => ">[-]<[>+<[-]]<[>>[-]+<<[-]]>>[-<<+>>]<",
                });
                self.code.push('<');
            }
            Unary { op, right } => {
                use ast::UnaryOpKind::*;

                match op.kind {
                    Plus => self.generate_expr(right)?,
                    Minus => {
                        // "[-]>{right}[-<->]<"
                        self.code.push_str("[-]>");
                        self.stack_ptr += 1;
                        self.generate_expr(right)?;
                        self.stack_ptr -= 1;
                        self.code.push_str("[-<->]<");
                    }
                    Not => {
                        // "[-]+>{right}[<->[-]]<"
                        self.code.push_str("[-]+>");
                        self.stack_ptr += 1;
                        self.generate_expr(right)?;
                        self.stack_ptr -= 1;
                        self.code.push_str("[<->[-]]<");
                    }
                }
            }
            Call { name, args } => self.generate_call(name, &args, true)?,
            Var { name } => {
                let rel_addr = match self.lookup_var(&name.value) {
                    Some(addr) => self.stack_ptr - addr,
                    None => {
                        return compiler_error(
                            name.pos,
                            format!("undeclared variable `{}`", name.value),
                        )
                    }
                };
                write!(
                    self.code,
                    "[-]>[-]<{left}[-{right}+>+<{left}]{right}>[-<{left}+{right}>]<",
                    left = "<".repeat(rel_addr),
                    right = ">".repeat(rel_addr),
                )
                .unwrap();
            }
            Int { value, .. } => {
                self.code.push_str("[-]");
                for _ in 0..*value {
                    self.code.push('+');
                }
            }
        }
        Ok(())
    }

    fn generate_call(
        &mut self,
        name: &ast::Ident,
        args: &[ast::Expr],
        expr: bool,
    ) -> CompilerResult<()> {
        if let Some(current) = self.current_function.as_ref() {
            if current == &name.value {
                return compiler_error(name.pos, format!("recursive function `{}`", name.value));
            }
        }
        let func = match self.functions.get(&name.value) {
            Some(func) => func,
            None => {
                return compiler_error(name.pos, format!("undefined function `{}`", name.value))
            }
        };
        if expr && func.void {
            return compiler_error(
                name.pos,
                format!("function `{}` has return type void", name.value),
            );
        }
        if args.len() != func.arity {
            return compiler_error(
                name.pos,
                format!("expected {} arguments, got {}", args.len(), func.arity),
            );
        }
        let code = func.code.clone(); // workaround, `generate_expr` needs `&mut self`
        for arg in args {
            self.generate_expr(arg)?;
            self.code.push('>');
            self.stack_ptr += 1;
        }
        for _ in 0..args.len() {
            self.code.push('<');
        }
        self.stack_ptr -= args.len();
        self.code.push_str(&code);
        Ok(())
    }
}

/// Generates a [`Program`] from an [`ast::Program`], while doing semantic analysis.
pub fn generate_code(
    ast: ast::Program,
    std: Option<Program>,
    optimize: bool,
) -> CompilerResult<Program> {
    use ast::Item::*;

    let mut code_gen = CodeGen::new();
    if let Some(std_program) = std {
        code_gen.functions = std_program.functions;
    }
    for item in ast.items {
        match item {
            Function(func) => code_gen.generate_function(func)?,
            Statement(stmt) => code_gen.generate_statement(stmt)?,
        }
    }

    if optimize {
        for func in code_gen.functions.values_mut() {
            optimize_code(&mut func.code);
        }
        optimize_code(&mut code_gen.code);
    }

    Ok(Program {
        name: ast.name,
        functions: code_gen.functions,
        code: code_gen.code,
    })
}
