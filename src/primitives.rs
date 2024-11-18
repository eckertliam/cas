use crate::{ast::Expression, compiler::compile_expr};

fn add(exprs: &[Expression]) -> Result<String, String> {
    let mut instructions = exprs.iter().map(|e| compile_expr(e)).collect::<Result<Vec<String>, String>>()?;
    for _ in 0..instructions.len() - 1 {
        instructions.push("f64.add".to_string());
    }
    Ok(instructions.join("\n"))
}

#[cfg(test)]
mod tests {
    use crate::pos::Pos;

    use super::*;

    #[test]
    fn test_add() {
        let exprs = vec![Expression::new_number(1.0, Pos::default()), Expression::new_number(2.0, Pos::default()), Expression::new_number(3.0, Pos::default())];
        let result = add(&exprs).unwrap();
        assert_eq!(result, "f64.const 1\nf64.const 2\nf64.const 3\nf64.add\nf64.add");
    }
}
