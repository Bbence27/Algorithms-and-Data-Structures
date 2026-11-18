pub fn is_correct_parenthesis(input: &str) -> bool {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}
