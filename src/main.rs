use std::cell::RefCell;

#[derive(Debug)]
struct Thing<'a> {
    name: RefCell<&'a str>,
}

fn main() {
    let mut secret = "shala ma cookie".to_string();
    let mut t = Thing {
        name: RefCell::new(&secret),
    };
    secret = "yoyo".to_string();
    dbg!(&t);
}
