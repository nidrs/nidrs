echo "#![feature(print_internals)]" > ../example-expand/src/main.rs
echo "#![feature(panic_internals)]" >> ../example-expand/src/main.rs
echo "#![feature(alloc)]" >> ../example-expand/src/main.rs
echo "#![feature(fmt_helpers_for_derive)]" >> ../example-expand/src/main.rs
echo "#![allow(warnings, unused)]" >> ../example-expand/src/main.rs
cargo expand >> ../example-expand/src/main.rs
echo "extern crate alloc;" >> ../example-expand/src/main.rs