//infix to postfix conversion
fn priority(ch: char) -> u8 {
    match ch {
        '(' | '\n' => 0,
        '+' | '-' => 1,
        '/' | '*' => 2,
        '$' => 3,
        _ => 4,
    }
}

fn main() {
    let mut infix_exp = String::new();
    let mut postfix_exp = String::new();
    let mut stack: Vec<char> = Vec::new();
    println!("Enter infix expression:");
    std::io::stdin()
        .read_line(&mut infix_exp)
        .expect("could not read input");

    for ch in infix_exp.chars() {
        if ch == '(' {
            stack.push('(');
        } else if ch == ')' {
            let mut popped: char;
            loop {
                popped = match stack.pop() {
                    Some(x) => x,
                    None => panic!("invalid exp supplied"),
                };
                if popped == '(' {
                    break;
                } else {
                    postfix_exp.push(popped);
                }
            }
        } else if priority(ch) < 4 {
            let curr_priority = priority(ch);
            while stack.len() > 0 && priority(stack[stack.len() - 1]) >= curr_priority {
                if let Some(x) = stack.pop() {
                    postfix_exp.push(x);
                }
            }
            stack.push(ch);
        } else {
            postfix_exp.push(ch);
        }
    }
    println!("Result: {}", postfix_exp);
}
