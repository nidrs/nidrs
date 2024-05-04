echo "#![feature(print_internals)]" > ./src/test.rs
echo "#![feature(panic_internals)]" >> ./src/test.rs
echo "#![feature(alloc)]" >> ./src/test.rs
echo "#![feature(fmt_helpers_for_derive)]" >> ./src/test.rs
echo "#![allow(warnings, unused)]" >>  ./src/test.rs
cargo expand >> ./src/test.rs
echo "extern crate alloc;" >> ./src/test.rs