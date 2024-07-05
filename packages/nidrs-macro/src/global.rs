use std::mem;
use std::{any::Any, cell::RefCell, collections::HashMap};

struct Globals {
    pub data: HashMap<String, Box<dyn Any>>,
}

static mut GLOBALS: Option<&mut Globals> = None;

pub fn set<T: Any>(value: T) {
    unsafe {
        if let Some(globals) = GLOBALS.as_mut() {
            globals.data.insert(std::any::type_name::<T>().to_string(), Box::new(value));
        } else {
            let mut new_globals = Globals { data: HashMap::new() };
            new_globals.data.insert(std::any::type_name::<T>().to_string(), Box::new(value));
            GLOBALS = Some(Box::leak(Box::new(new_globals)));
        }
    }
}

// pub fn get<T: Any>() -> Option<&'static mut T> {
//     unsafe {
//         let mut globals = GLOBALS.as_ref().unwrap();
//         let value = globals.data.get_mut(&std::any::type_name::<T>().to_string()).unwrap();

//         mem::transmute(value)
//     }
// }
