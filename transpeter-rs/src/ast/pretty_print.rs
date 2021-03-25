//! Pretty printing for the AST.

use super::*;

/// Pretty prints the given AST.
pub fn pretty_print_ast(program: &Program) {
    fn pretty_print_item(item: &Item, prefix: &str) {
        use Item::*;

        match item {
            Function(ItemFunction {
                name,
                return_type,
                parameters,
                statements,
            }) => {
                println!("{}Function", prefix);
                println!("{}  name: {}", prefix, name.value);
                println!("{}  return type: {}", prefix, return_type.value);
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

    fn pretty_print_declaration(decl: &Declaration, prefix: &str) {
        println!("{}Declaration", prefix);
        println!("{}  type: {}", prefix, decl.type_.value);
        println!("{}  name: {}", prefix, decl.name.value);
        if let Some(init) = &decl.init {
            println!("{}  init:", prefix);
            pretty_print_expr(init, &format!("{}    ", prefix));
        }
    }

    fn pretty_print_statement(stmt: &Statement, prefix: &str) {
        use Statement::*;

        match stmt {
            Declaration(decl) => pretty_print_declaration(decl, prefix),
            Block(statements) => {
                println!("{}Block", prefix);
                let new_prefix = format!("{}  ", prefix);
                for stmt in statements {
                    pretty_print_statement(stmt, &new_prefix);
                }
            }
            If {
                condition,
                then_statement,
                else_statement,
                ..
            } => {
                println!("{}If", prefix);
                println!("{}  condition:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(condition, &new_prefix);
                println!("{}  then statement:", prefix);
                pretty_print_statement(then_statement, &new_prefix);
                if let Some(stmt) = else_statement {
                    println!("{}  else statement:", prefix);
                    pretty_print_statement(stmt, &new_prefix);
                }
            }
            While {
                condition,
                statement,
                ..
            } => {
                println!("{}While", prefix);
                println!("{}  condition:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(condition, &new_prefix);
                println!("{}  statement:", prefix);
                pretty_print_statement(statement, &new_prefix);
            }
            Repeat {
                expr, statement, ..
            } => {
                println!("{}Repeat", prefix);
                println!("{}  expression:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(expr, &new_prefix);
                println!("{}  statement:", prefix);
                pretty_print_statement(statement, &new_prefix);
            }
            Return { expr, .. } => {
                println!("{}Return", prefix);
                pretty_print_expr(expr, &format!("{}  ", prefix));
            }
            Inline { code, .. } => {
                println!("{}Inline", prefix);
                println!("{}  {}", prefix, std::str::from_utf8(code).unwrap());
            }
            Assign { name, op, expr } => {
                println!("{}Assign", prefix);
                println!("{}  name: {}", prefix, name.value);
                println!("{}  operator: {}", prefix, op.kind);
                println!("{}  expression:", prefix);
                pretty_print_expr(expr, &format!("{}  ", prefix));
            }
            Call { name, args } => {
                println!("{}Call", prefix);
                println!("{}  name: {}", prefix, name.value);
                println!("{}  arguments:", prefix);
                let new_prefix = format!("{}    ", prefix);
                for arg in args {
                    pretty_print_expr(arg, &new_prefix);
                }
            }
        }
    }

    fn pretty_print_expr(expr: &Expr, prefix: &str) {
        use Expr::*;

        match expr {
            Binary { left, op, right } => {
                println!("{}Binary", prefix);
                println!("{}  left:", prefix);
                let new_prefix = format!("{}    ", prefix);
                pretty_print_expr(left, &new_prefix);
                println!("{}  operator: {}", prefix, op.kind);
                println!("{}  right:", prefix);
                pretty_print_expr(right, &new_prefix);
            }
            Unary { op, right } => {
                println!("{}Unary", prefix);
                println!("{}  operator: {}", prefix, op.kind);
                println!("{}  right:", prefix);
                pretty_print_expr(right, &format!("{}    ", prefix));
            }
            Call { name, args } => {
                println!("{}Call", prefix);
                println!("{}  name: {}", prefix, name.value);
                println!("{}  arguments:", prefix);
                let new_prefix = format!("{}    ", prefix);
                for arg in args {
                    pretty_print_expr(arg, &new_prefix);
                }
            }
            Move { name } => {
                println!("{}Move {}", prefix, name.value);
            }
            Var { name } => {
                println!("{}Var {}", prefix, name.value);
            }
            Int { value, .. } => {
                println!("{}Int {}", prefix, value);
            }
        }
    }

    println!("Program");
    println!("  name: {}", program.name);
    println!("  items:");
    for item in &program.items {
        pretty_print_item(item, "    ");
    }
}
