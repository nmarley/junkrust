enum List {
    Cons(i32, Box<List>),
    None,
}

impl List {
    pub fn new() -> Self {
        List::None
    }

    pub fn add(&mut self, num: i32) {
        let mut r = self;
        while !r.is_none() {
            r = self;
        }
        r = List::Cons(num, Box::new(List::None));
    }

    pub fn is_none(&self) -> bool {
        self == List::None
    }
}

fn main() {
    let mut list = List::new();
}
