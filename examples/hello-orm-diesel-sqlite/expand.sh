echo "#![feature(print_internals)]" > ./examples/expand.rs
echo "#![feature(panic_internals)]" >> ./examples/expand.rs
echo "#![feature(alloc)]" >> ./examples/expand.rs
echo "#![feature(fmt_helpers_for_derive)]" >> ./examples/expand.rs
echo "#![allow(warnings, unused)]" >>  ./examples/expand.rs
echo "#![feature(hint_must_use)]" >>  ./examples/expand.rs
echo "#![feature(liballoc_internals)]" >>  ./examples/expand.rs
cargo expand >> ./examples/expand.rs
echo "extern crate alloc;" >> ./examples/expand.rs