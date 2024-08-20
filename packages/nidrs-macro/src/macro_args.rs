use std::collections::HashMap;

///
/// data:
///   vec = [Arg::Int(None), Arg::Int(None), Arg::Int(None)];
/// def:
///   co = `$(t1: Int, t2: Int)`;
///   fm = Formal::define().arg(Arg::Int(None), "参数 1", true);
/// parse:
///   fm.parse("$(1, 2, 3)");
///

pub struct Formal {
    pub defs: Vec<ArgDef>,
}

impl Formal {
    pub fn new() -> Self {
        Formal { defs: vec![] }
    }

    pub fn def(mut self, arg: ArgType, desc: &str, required: bool) -> Self {
        self.defs.push(ArgDef::new(arg, desc, required));
        self
    }

    pub fn parse(&self, input: &str) -> Vec<ArgValue> {
        let mut res: Vec<ArgValue> = vec![];
        let input = expr_fix(input);
        let expr = syn::parse_str::<syn::ExprCall>(&input).unwrap();
        println!("{:#?}", expr.args);

        for arg in expr.args {
            match arg {
                syn::Expr::Lit(lit) => match lit.lit {
                    syn::Lit::Int(int) => {
                        res.push(ArgValue::Int(int.base10_parse::<i32>().unwrap()));
                    }
                    syn::Lit::Str(str) => {
                        res.push(ArgValue::String(str.value()));
                    }
                    _ => {}
                },
                syn::Expr::Path(path) => {
                    res.push(ArgValue::Ident(path.path.segments[0].ident.to_string()));
                }
                _ => {}
            }
        }

        res
    }
}

pub struct ArgDef {
    pub arg_type: ArgType,
    pub desc: String,
    pub required: bool,
}

impl ArgDef {
    pub fn new(arg_type: ArgType, desc: &str, required: bool) -> Self {
        ArgDef { arg_type: arg_type, desc: desc.to_string(), required: required }
    }
}

#[derive(Debug)]
pub enum ArgValue {
    Null,
    Ident(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Object(HashMap<String, ArgValue>),
    Array(Vec<ArgValue>),
}

#[derive(Debug)]
pub enum ArgType {
    Null,
    Ident,
    Int,
    Float,
    Bool,
    String,
    Object(HashMap<String, ArgValue>),
    Array(Vec<ArgValue>),
}

fn expr_fix(input: &str) -> String {
    let mut peek = input.chars().peekable();
    let mut output = String::new();

    while let Some(cur) = peek.next() {
        let next = peek.peek().copied();
        if let Some(next) = next {
            if next == '{' && !cur.is_alphabetic() {
                output.push(cur);
                output.push_str("O")
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
    fn test_formal() {
        // F(Int, Int, Int)
        // F(Int)
        // F(Module({ a: Int, b: Int }))
        let mut fm = Formal::new().def(ArgType::Ident, "参数 1", true);
        // fm.parse("F(Object{ a: Int, b: Optional(Int) }, Array(Int))");
        let value = fm.parse("F(Test)");
        println!("{:?}", value);
        // assert_eq!(format!("{:?}", fm.args), format!("{:?}", vec![Arg::Int(Some(1)), Arg::Int(Some(2)), Arg::Int(Some(3))]));
    }

    #[test]
    fn test_expr_fix() {
        let input = "F(1, 2, { a: { b:2 } })";
        let output = expr_fix(input);
        assert_eq!(output, "F(1, 2, O{ a: O{ b:2 } })");
    }
}
