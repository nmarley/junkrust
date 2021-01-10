use rand::Rng;

fn main() {
    // let mut bv: Vec<u8> = Vec::with_capacity((((u32::MAX - 7) / 8) + 1) as usize);
    let mut bv: Vec<u8> = vec![0; (((u32::MAX - 7) / 8) + 1) as usize];
    println!("bv[0] = {:08b}", bv[0]);

    let randos: Vec<u32> = vec![
        38405, 33022, 560, 59256, 60596, 32679, 9642, 31328, 64408, 55685,
    ];

    load_bit_vector(&mut bv, &randos);
    println!("bv[4800] = {:08b}", bv[4800]);
}

fn load_bit_vector(bv: &mut [u8], randos: &[u32]) {
    for rando in randos {
        let byte_index = rando / 8;
        let bit_index = match rando % 8 {
            0 => 0,
            modulus => 1 << (modulus - 1),
        } as u8;
        bv[byte_index as usize] |= bit_index;
    }
}

#[allow(dead_code)]
fn gen_randos() {
    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        println!("{}", rng.gen_range(0..u32::MAX));
    }
}
