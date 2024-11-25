use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("Uppercase chars won't be empty in this context."),
        );
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("Lowercase chars won't be empty in this context."),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("Number chars won't be empty in this context."),
        );
    }
    if symbols {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("Symbol chars won't be empty in this context."),
        );
    }

    for _ in 0..length {
        let char = chars
            .choose(&mut rng)
            .expect("Chars won't be empty in this context.");
        password.push(*char);
    }
    password.shuffle(&mut rng);
    println!("Your password is: {}", String::from_utf8(password)?);

    Ok(())
}
