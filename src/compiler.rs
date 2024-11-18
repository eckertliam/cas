use crate::ast::Expression;

pub fn compile_expr(expr: &Expression) -> Result<String, String> {
    match expr {
        Expression::Number(n) => Ok(format!("f64.const {}", n.value)),
        _ => unimplemented!("Unsupported expression: {:?}", expr),
    }
}
