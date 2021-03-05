//! Pretty printing for the AST.

use crate::ast::*;

pub fn pretty_print_ast(program: Program) {
    fn pretty_print_item(item: Item, prefix: &str) {
        use ItemKind::*;
        match item.kind {
            Function {
                name,
                return_type,
                parameters,
                statements,
            } => {
                println!("{}Function", prefix);
                println!("{}  name: {}", prefix, name.name);
                println!("{}  return type: {}", prefix, return_type.name);
                println!("{}  parameters:", prefix);
                let new_prefix = format!("{}    ", prefix);
                for param in parameters {
                    pretty_print_declaration(param, &new_prefix);
                }
                println!("{}  statements:", prefix);
                for stmt in statements {
                    pretty_print_statement(stmt, &new_prefix);
                }
            }
            Statement(stmt) => pretty_print_statement(stmt, prefix),
        }
    }

    fn pretty_print_declaration(decl: Declaration, prefix: &str) {
        println!("{}Declaration", prefix);
        println!("{}  type: {}", prefix, decl.type_.name);
        println!("{}  name: {}", prefix, decl.name.name);
        if let Some(init) = decl.init {
            println!("{}  init:", prefix);
            pretty_print_expr(init, &format!("{}    ", prefix));
        }
    }

    fn pretty_print_statement(stmt: Statement, prefix: &str) {
        use StatementKind::*;
        match stmt.kind {
            Declaration(decl) => pretty_print_declaration(decl, prefix),
            Block { statements } => {
                println!("{}Block", prefix);
                let new_prefix = format!("{}  ", prefix);
                for stmt in statements {
                    pretty_print_statement(stmt, &new_prefix);
                }
            }
            If {
                condition,
                if_statement,
                else_statement,
            } => {
                println!("{}If", prefix);
                println!("{}  condition:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(condition, &new_prefix);
                println!("{}  if statement:", prefix);
                pretty_print_statement(*if_statement, &new_prefix);
                if let Some(stmt) = else_statement {
                    println!("{}  else statement:", prefix);
                    pretty_print_statement(*stmt, &new_prefix);
                }
            }
            While {
                condition,
                statement,
            } => {
                println!("{}While", prefix);
                println!("{}  condition:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(condition, &new_prefix);
                println!("{}  statement:", prefix);
                pretty_print_statement(*statement, &new_prefix);
            }
            Repeat { expr, statement } => {
                println!("{}Repeat", prefix);
                println!("{}  expression:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(expr, &new_prefix);
                println!("{}  statement:", prefix);
                pretty_print_statement(*statement, &new_prefix);
            }
            Return { expr } => {
                println!("{}Return", prefix);
                pretty_print_expr(expr, &format!("{}  ", prefix));
            }
            Inline { code } => {
                println!("{}Inline", prefix);
                println!("{}  {}", prefix, code);
            }
            Assign { var, op, expr } => {
                println!("{}Assign", prefix);
                println!("{}  name: {}", prefix, var.name);
                println!("{}  operator: {:?}", prefix, op);
                println!("{}  expression:", prefix);
                pretty_print_expr(expr, &format!("{}  ", prefix));
            }
            Call { name, args } => {
                println!("{}Call", prefix);
                println!("{}  name: {}", prefix, name.name);
                println!("{}  arguments:", prefix);
                let new_prefix = format!("{}    ", prefix);
                for arg in args {
                    pretty_print_expr(arg, &new_prefix);
                }
            }
        }
    }

    fn pretty_print_expr(expr: Expr, prefix: &str) {
        use ExprKind::*;
        match expr.kind {
            Binary { left, op, right } => {
                println!("{}Binary", prefix);
                println!("{}  left:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(*left, &new_prefix);
                println!("{}  operator: {:?}", prefix, op);
                println!("{}  right:", prefix);
                pretty_print_expr(*right, &new_prefix);
            }
            Unary { op, right } => {
                println!("{}Binary", prefix);
                println!("{}  operator: {:?}", prefix, op);
                println!("{}  right:", prefix);
                pretty_print_expr(*right, &format!("{}    ", prefix));
            }
            Call { name, args } => {
                println!("{}Call", prefix);
                println!("{}  name: {}", prefix, name.name);
                println!("{}  arguments:", prefix);
                let new_prefix = format!("{}    ", prefix);
                for arg in args {
                    pretty_print_expr(arg, &new_prefix);
                }
            }
            Var { name } => {
                println!("{}Var", prefix);
                println!("{}  {}", prefix, name.name);
            }
            Int { value, .. } => {
                println!("{}Int", prefix);
                println!("{}  {}", prefix, value);
            }
        }
    }

    println!("Program");
    println!("  name: {}", program.name);
    println!("  items:");
    for item in program.items {
        pretty_print_item(item, "    ");
    }
}
