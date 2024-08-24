use crate::macro_args::Value;
use syn::Error;

pub trait ArgsParse
where
    Self: Sized,
{
    fn parse(args: Vec<Value>) -> Result<Self, Error>;
}
