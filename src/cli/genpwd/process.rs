use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;

use super::{LOWER, NUMBER, SYMBOL, UPPER};

pub fn process_genpwd(
    length: u8,
    uppercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    //TODO: implement the password generation logic
    // println!("{:#?}", opts);
    let mut rng = rand::rng();
    // let mut pwd = String::new();
    let mut pwd = Vec::new();
    let mut chars = Vec::new();
    chars.extend_from_slice(LOWER);
    pwd.push(*LOWER.choose(&mut rng).unwrap());

    if uppercase {
        chars.extend_from_slice(UPPER);
        pwd.push(*UPPER.choose(&mut rng).unwrap());
    }
    if number {
        chars.extend_from_slice(NUMBER);
        pwd.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        pwd.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    for _ in 0..(length - pwd.len() as u8) {
        pwd.push(*chars.choose(&mut rng).unwrap());
    }
    pwd.shuffle(&mut rng);
    let pwd = String::from_utf8_lossy(&pwd);
    println!("{}", pwd);
    Ok(())
}
