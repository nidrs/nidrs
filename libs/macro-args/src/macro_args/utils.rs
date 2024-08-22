pub(crate) fn expr_fix(input: &str) -> String {
    let mut peek = input.chars().peekable();
    let mut output = String::new();

    while let Some(cur) = peek.next() {
        let next = peek.peek().copied();
        if let Some(next) = next {
            if next == '{' && !cur.is_alphabetic() {
                output.push(cur);
                output.push('O')
            } else {
                output.push(cur);
            }
        } else {
            output.push(cur);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_fix() {
        let input = "F(1, 2, { a: { b:2 } })";
        let output = expr_fix(input);
        assert_eq!(output, "F(1, 2, O{ a: O{ b:2 } })");
    }
}
