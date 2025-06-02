#[derive(Debug)]
struct Quad {
    operator: String, 
    arg1: Option<String>, 
    arg2: Option<String>, 
    result: String,       
}

fn generate_quads(stmt: &Statement, quads: &mut Vec<Quad>, temp_counter: &mut i32) -> String {
    match stmt {
        Statement::Variable { id, value } => {
            let result = generate_expression_quads(value, quads, temp_counter);
            quads.push(Quad {
                operator: "=".to_string(),
                arg1: Some(result),
                arg2: None,
                result: id.clone(),
            });
            id.clone()
        }
        _ => unimplemented!(),
    }
}

fn generate_expression_quads(expr: &Expression, quads: &mut Vec<Quad>, temp_counter: &mut i32) -> String {
    match expr {
        Expression::Integer(n) => n.to_string(),
        Expression::Float(n) => n.to_string(),
        Expression::Variable(id) => id.clone(),
        Expression::Binary { left, operator, right } => {
            let arg1 = generate_expression_quads(left, quads, temp_counter);
            let arg2 = generate_expression_quads(right, quads, temp_counter);
            let temp = format!("t{}", *temp_counter);
            *temp_counter += 1;
            quads.push(Quad {
                operator: format!("{:?}", operator).to_lowercase(),
                arg1: Some(arg1),
                arg2: Some(arg2),
                result: temp.clone(),
            });
            temp
        }
    }
}