use std::ops::{Deref, DerefMut};

use crate::Transform;

use super::*;

mod null;
pub use null::*;

mod options;
pub use options::*;

mod string;
pub use string::*;

mod array;
pub use array::*;

mod object;
pub use object::*;

mod expr;
pub use expr::*;

mod bool;
pub use bool::*;

mod float;
pub use float::*;

mod int;
pub use int::*;

mod extends;
pub use extends::*;

mod native;
