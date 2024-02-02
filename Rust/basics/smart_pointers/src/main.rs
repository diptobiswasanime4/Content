enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    for new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn main_list() {
    let list = Cons(11, Box::new(Cons(23, Box::new(Cons(33, Box::new(Nil))))));
}

fn main_box() {
    let bx = Box::new(5);
    println!("{bx}");
}
