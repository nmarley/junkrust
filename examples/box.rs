#[derive(PartialEq)]
enum List {
    Cons(i32, Box<List>),
    None,
}

impl List {
    pub fn new(n: i32) -> Self {
        List::Cons(n, Box::new(List::None))
    }

    pub fn add(&mut self, n: i32) {
        if self.is_none() {
            *self = List::new(n)
        }
    }

    pub fn is_none(&self) -> bool {
        *self == List::None
    }
}

fn main() {
    let mut list = List::None;
    println!("list.is_none(): {}", list.is_none());

    list.add(3);
    println!("list.is_none(): {}", list.is_none());

    list = List::new(7);
    println!("list.is_none(): {}", list.is_none());
}
