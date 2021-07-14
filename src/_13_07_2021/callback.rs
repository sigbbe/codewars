#[allow(dead_code)]
pub struct Processor<'a> {
    callback: Box<dyn FnMut() + 'a>,
}

pub trait ProcessorTrait<'a> {
    fn set_callback(&mut self, c: impl FnMut() + 'a);
    fn process_events(&mut self);
}

impl<'a> ProcessorTrait<'a> for Processor<'a> {
    #[allow(dead_code)]
    fn set_callback(&mut self, c: impl FnMut() + 'a) {
        self.callback = Box::new(c);
    }
    #[allow(dead_code)]
    fn process_events(&mut self) {
        (self.callback)();
    }
}

impl<'a> Processor<'a> {
    #[allow(dead_code)]
    pub fn new(c: impl FnMut() + 'a) -> Self {
        Self {
            callback: Box::new(c),
        }
    }
}

#[allow(dead_code)]
fn simple_callback() {
    println!("hello");
}

#[allow(dead_code)]
pub fn run() {
    let mut p = Processor {
        callback: Box::new(simple_callback),
    };
    let s = "world!".to_string();
    p.process_events();
    let callback2 = move || println!("hello {}", s);
    p.set_callback(callback2);
    p.process_events();
}

use std::any::type_name;
#[allow(dead_code)]
fn print_typeof<T: ?Sized>(_: &T) {
    println!("{}", type_name::<T>());
}
