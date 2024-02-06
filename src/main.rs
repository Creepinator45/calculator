use std::io;

#[derive(Debug, PartialEq, Clone)]
enum PlusMinus {
    Positive,
    Negative
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Parenthetical(PlusMinus),
    Exponentiation(PlusMinus),
    Multiplication(PlusMinus),
    Addition(PlusMinus),
    Num(f32),
}

fn string_to_tokens(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for raw_token in input.split(' ') {
        if let Ok(val) = raw_token.parse::<f32>() {
            tokens.push(Token::Num(val))
        }
        else {
            match raw_token {
                "(" => tokens.push(Token::Parenthetical(PlusMinus::Positive)),
                ")" => tokens.push(Token::Parenthetical(PlusMinus::Negative)),
                "^" => tokens.push(Token::Exponentiation(PlusMinus::Positive)),
                "*" => tokens.push(Token::Multiplication(PlusMinus::Positive)),
                "/" => tokens.push(Token::Multiplication(PlusMinus::Negative)),
                "+" => tokens.push(Token::Addition(PlusMinus::Positive)),
                "-" => tokens.push(Token::Addition(PlusMinus::Negative)),
                val => println!("Ignored unknown token: {}", val)
            }
        }
    }
    return tokens
}

fn evaluate(mut expression: Vec<Token>) -> Token{

    println!("evaluating {:?} for parenthesis", &expression);
    //evaluate all parenthesis
    while let Some(index) = expression.iter().position(|x| x == &Token::Parenthetical(PlusMinus::Positive)) {
        println!("evaluating parenthesis");
        //scan for matching closing parenthesis
        let closing_index: usize = {
            let mut paren_count = 1;
            let mut pointer = index;
            while paren_count > 0 {
                pointer += 1;
                match expression.get(pointer) {
                    None => panic!("missing closing parenthesis"),
                    Some(Token::Parenthetical(PlusMinus::Positive)) => paren_count += 1,
                    Some(Token::Parenthetical(PlusMinus::Negative)) => paren_count -= 1,
                    Some(_) => (),
                }
            }
            pointer
        };
        let sub_expression = evaluate(expression[index+1..closing_index].to_vec());
        expression.splice(index..closing_index + 1, vec![sub_expression]);
    }

    println!("evaluating {:?} for exponents", &expression);
    //evaluate all exponents
    while let Some(index) = expression.iter().position(|x| x == &Token::Exponentiation(PlusMinus::Positive)) {
        println!("evaluating exponent");
        let Token::Num(val1) = expression[index-1] else { panic!("missing number") };
        let Token::Num(val2) = expression[index+1] else { panic!("missing number") };
        let sub_expression = Token::Num(val1.powf(val2));
        expression.splice(index-1..index+2, vec![sub_expression]);
    }

    println!("evaluating {:?} for multiplicaiton", &expression);
    //evaluate all multiplication/division
    while let Some(index) = expression.iter().position(|x| {if let Token::Multiplication(_) = x {true} else {false}}) {
        println!("evaluating multiplication/division");
        let Token::Num(val1) = expression[index-1] else { panic!("missing number") };
        let Token::Num(val2) = expression[index+1] else { panic!("missing number") };
        let sub_expression = Token::Num(match expression[index] {
            Token::Multiplication(PlusMinus::Positive) => val1 * val2,
            Token::Multiplication(PlusMinus::Negative) => val1 / val2,
            _ => panic!() //this case should be impossible to reach
        });
        expression.splice(index-1..index+2, vec![sub_expression]);
    }

    println!("evaluating {:?} for addition", &expression);
    //evaluate all addition/subtraction
    while let Some(index) = expression.iter().position(|x| {if let Token::Addition(_) = x {true} else {false}}) {
        println!("evaluating addition/subtraction");
        let Token::Num(val1) = expression[index-1] else { panic!("missing number") };
        let Token::Num(val2) = expression[index+1] else { panic!("missing number") };
        let sub_expression = Token::Num(match expression[index] {
            Token::Addition(PlusMinus::Positive) => val1 + val2,
            Token::Addition(PlusMinus::Negative) => val1 - val2,
            _ => panic!() //this case should be impossible to reach
        });
        expression.splice(index-1..index+2, vec![sub_expression]);
    }

    println!("outputting {:?}", &expression);
    return expression[0].clone()
}

fn main() {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    evaluate(string_to_tokens(user_input.trim().to_string()));
}