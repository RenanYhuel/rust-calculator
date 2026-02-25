
use std::{io};
use std::io::Write;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Expression {
    op1: f64,
    operator: Operation,
    op2: f64,
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Erreur de flush");
        let input = lire_input();
        if input.trim() == "exit" {
            break;
        }
        match parser(&input) {
            Ok(expression) => match compute(&expression) {
                Ok(result) => afficher(&expression, result),
                Err(e) => println!("Erreur de calcul: {}", e),
            },
            Err(e) => println!("Erreur: {}", e),
        }
    }
}


fn lire_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture");
    input.trim().to_string()
}

fn parser(input: &str) -> Result<Expression, String> {
    let input = input.trim();

    let operator_char = if input.contains('+') {
        '+'
    } else if input.contains('-') {
        '-'
    } else if input.contains('*') {
        '*'
    } else if input.contains('/') {
        '/'
    } else {
        return Err("Aucun opérateur valide trouvé".to_string());
    };

    let parts: Vec<&str> = input.split(operator_char).collect();
    if parts.len() != 2 {
        return Err("Format invalide, deux opérandes attendues".to_string());
    }

    let op1 = parts[0].trim().parse::<f64>()
        .map_err(|_| "Impossible de convertir le premier opérande".to_string())?;
    let op2 = parts[1].trim().parse::<f64>()
        .map_err(|_| "Impossible de convertir le deuxième opérande".to_string())?;

    let operator = match operator_char {
        '+' => Operation::Add,
        '-' => Operation::Subtract,
        '*' => Operation::Multiply,
        '/' => Operation::Divide,
        _ => unreachable!(),
    };

    Ok(Expression { op1, operator, op2 })
}

fn compute(expression: &Expression) -> Result<f64, String> {
    let op1 = expression.op1;
    let op2 = expression.op2;
    match expression.operator {
        Operation::Add => Ok(op1 + op2),
        Operation::Divide => {
            if op2 == 0.0 {
                Err("Division par zéro".to_string())
            } else {
                Ok(op1 / op2)
            }
        },
        Operation::Multiply => Ok(op1 * op2),
        Operation::Subtract => Ok(op1 - op2),
    }
}

fn afficher(expression: &Expression, result: f64) {
    let operator_symbol = match expression.operator {
        Operation::Add => '+',
        Operation::Subtract => '-',
        Operation::Multiply => '*',
        Operation::Divide => '/',
    };

    println!("{} {} {} = {}", expression.op1, operator_symbol, expression.op2, result);
}