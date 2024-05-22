use anstyle::{AnsiColor, Effects};
use clap::{arg, builder::styling::Styles, error::ErrorKind, value_parser, Command};
use rand::Rng;

fn main() {
    let styles = {
        Styles::styled()
            .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
            .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
            .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
            .placeholder(AnsiColor::Cyan.on_default())
            .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
            .valid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
            .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
    };

    let mut cmd = Command::new("genpasswd")
        .styles(styles)
        .bin_name("genpasswd")
        .arg(
            arg!(-n --sequences <NUM> "The number of alphanumeric sequences")
                .required(false)
                .default_value("3")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-l --length <NUM> "The length of each sequence")
                .required(false)
                .default_value("6")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-d --digits <NUM> "The total number of digits")
                .required(false)
                .default_value("1")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-u --uppercase <NUM> "The total number of uppercase letters")
                .required(false)
                .default_value("1")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-s --separator <CHAR> "The separator character")
                .required(false)
                .default_value("-")
                .value_parser(value_parser!(String)),
        );

    let matches = cmd.get_matches_mut();

    let count = *matches.get_one::<usize>("sequences").unwrap();
    let length = *matches.get_one::<usize>("length").unwrap();
    let digits = *matches.get_one::<usize>("digits").unwrap();
    let uppercase = *matches.get_one::<usize>("uppercase").unwrap();
    let separator = matches.get_one::<String>("separator").unwrap();

    match validate_parameters(count, length, digits, uppercase) {
        Ok(_) => {}
        Err(msg) => {
            let err = cmd.error(ErrorKind::ValueValidation, msg);
            err.exit();
        }
    }

    let password = generate_password(count, length, digits, uppercase, separator);
    println!("{}", password);
}

pub fn validate_parameters(
    count: usize,
    length: usize,
    digits: usize,
    uppercase: usize,
) -> Result<(), String> {
    let total_chars = count * length;
    if digits + uppercase > total_chars {
        return Err(String::from(
            "The number of uppercase letters and digits exceeds the total number of characters.",
        ));
    }
    Ok(())
}

pub fn generate_password(
    count: usize,
    length: usize,
    digits: usize,
    uppercase: usize,
    separator: &str,
) -> String {
    let mut rng = rand::thread_rng();
    let total_length = count * length;

    // Generate positions for digits and uppercase letters
    let digit_positions = generate_unique_positions(digits, total_length, &mut rng);
    let uppercase_positions = generate_unique_positions(uppercase, total_length, &mut rng);

    // Generate password sequences
    let mut password_chars: Vec<char> = Vec::with_capacity(total_length);

    for i in 0..total_length {
        if digit_positions.contains(&i) {
            password_chars.push(rng.gen_range('0'..='9'));
        } else if uppercase_positions.contains(&i) {
            password_chars.push(rng.gen_range('A'..='Z'));
        } else {
            password_chars.push(rng.gen_range('a'..='z'));
        }
    }

    // Split password into sequences and join with separator
    password_chars
        .chunks(length)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(separator)
}

fn generate_unique_positions(count: usize, max: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut positions = Vec::with_capacity(count);
    while positions.len() < count {
        let pos = rng.gen_range(0..max);
        if !positions.contains(&pos) {
            positions.push(pos);
        }
    }
    positions
}
