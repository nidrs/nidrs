use crate::macro_args::Value;
use syn::Error;

pub trait ArgsParse
where
    Self: Sized,
{
    fn parse(input: &str) -> Result<Self, Error>;
}
