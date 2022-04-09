use std::collections::VecDeque;

fn main() {
    let mut buff1: VecDeque<u8> = VecDeque::default();
    let mut buff2: VecDeque<u8> = VecDeque::default();

    buff1.push_back(1);
    buff1.push_back(2);
    buff1.push_back(3);

    std::mem::swap(&mut buff1, &mut buff2);

    println!("buff1: {:?}", buff1);
    println!("buff2: {:?}", buff2);
}
