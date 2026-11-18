pub fn is_correct_parenthesis(input: &str) -> bool {
    let mut copy = input.to_string();

    while !copy.is_empty() {
        let mut found = None;
        for pair in ["()", "[]", "{}"] {
            if let Some(pos) = copy.find(pair) {
                found = Some(pos..pos + 2);
                break;
            }
        }

        match found {
            Some(range) => copy.replace_range(range, ""),
            None => return false,
        }
    }

    true
}