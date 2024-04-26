echo "#![feature(print_internals)]" > ../hello-expand/src/main.rs
echo "#![feature(panic_internals)]" >> ../hello-expand/src/main.rs
echo "#![feature(alloc)]" >> ../hello-expand/src/main.rs
echo "#![feature(fmt_helpers_for_derive)]" >> ../hello-expand/src/main.rs
echo "#![allow(warnings, unused)]" >> ../hello-expand/src/main.rs
cargo expand >> ../hello-expand/src/main.rs
echo "extern crate alloc;" >> ../hello-expand/src/main.rs