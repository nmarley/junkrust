use rand::Rng;

fn main() {
    // let n = ((u8::MAX - 7) / 8) + 8;
    // println!("n = {}", n);
    // 8192
    let _n = (u16::MAX as u32 + 1) / 8;
    // println!("n = {}", n);

    let mut bv: [u8; 8192] = [0; 8192];
    println!("bv[0] = {}", bv[0]);

    let randos: Vec<u16> = vec![
        38405, 33022, 560, 59256, 60596, 32679, 9642, 31328, 64408, 55685,
    ];

    load_bit_vector(&mut bv, &randos);
    println!("bv[4800] = {}", bv[4800]);
}

fn load_bit_vector(bv: &mut [u8; 8192], randos: &[u16]) {
    for rando in randos {
        let byte_index = (rando / 8);
        let bit_index = 1 << ((rando % 8) - 1);
        bv[byte_index as usize] |= bit_index;
    }
}

#[allow(dead_code)]
fn gen_randos() {
    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        println!("{}", rng.gen_range(0..u16::MAX));
    }
}
