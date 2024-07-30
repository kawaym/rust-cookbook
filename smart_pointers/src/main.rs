use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let list = Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    );

    println!("{list:#?}");

    let x = 5;
    let y = Box::new(x);
    let w = 5;
    let z = MyBox::new(w);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Rust"));

    hello(&m);

    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let b = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(a);

    // let lista = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let listb = Cons(3, Rc::clone(&lista));
    // let listc = Cons(3, Rc::clone(&lista));
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
