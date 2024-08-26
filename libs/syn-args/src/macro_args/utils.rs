use syn::Error;

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

pub fn otr<T>(opt: Option<T>) -> Result<T, Error> {
    match opt {
        Some(val) => Ok(val),
        None => Err(Error::new(proc_macro2::Span::call_site(), "Invalid args otr")),
    }
}

pub fn ewc<F, T, E>(callback: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    // 调用闭包并返回结果
    callback()
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
