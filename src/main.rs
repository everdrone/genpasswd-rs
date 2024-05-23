use std::collections::HashSet;

use anstyle::{AnsiColor, Effects, Style};
use clap::{arg, builder::styling::Styles, error::ErrorKind, value_parser, Command};
use rand::Rng;

struct PasswordOptions {
    pub sequences: usize,
    pub length: usize,
    pub digits: usize,
    pub uppercase: usize,
    pub separator: String,
}

const GREEN_BOLD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const CYAN_BOLD: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const CYAN: Style = AnsiColor::Cyan.on_default();
const YELLOW_BOLD: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
const RED_BOLD: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);

fn main() {
    let styles = {
        Styles::styled()
            .header(GREEN_BOLD)
            .usage(GREEN_BOLD)
            .literal(CYAN_BOLD)
            .placeholder(CYAN)
            .error(RED_BOLD)
            .valid(YELLOW_BOLD)
            .invalid(YELLOW_BOLD)
    };

    let mut cmd = Command::new("genpasswd")
        .styles(styles)
        .bin_name("genpasswd")
        .version(clap::crate_version!())
        .about("Generate a strong password")
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

    let sequences = *matches.get_one::<usize>("sequences").unwrap();
    let length = *matches.get_one::<usize>("length").unwrap();
    let digits = *matches.get_one::<usize>("digits").unwrap();
    let uppercase = *matches.get_one::<usize>("uppercase").unwrap();
    let separator = matches.get_one::<String>("separator").unwrap().clone();

    let opts = PasswordOptions {
        sequences,
        length,
        digits,
        uppercase,
        separator,
    };

    match validate_parameters(&opts) {
        Ok(_) => {}
        Err(msg) => {
            let err = cmd.error(ErrorKind::ValueValidation, msg);
            err.exit();
        }
    }

    let password = generate_password(&opts);
    println!("{}", password);
}

fn validate_parameters(opts: &PasswordOptions) -> Result<(), String> {
    let total_chars = opts.sequences * opts.length;

    if opts.digits + opts.uppercase > total_chars {
        return Err(String::from(
            "The number of uppercase letters and digits exceeds the total number of characters.",
        ));
    }

    Ok(())
}

fn generate_password(opts: &PasswordOptions) -> String {
    let mut rng = rand::thread_rng();
    let total_length = opts.sequences * opts.length;

    // Generate positions for digits and uppercase letters
    let mut positions = HashSet::new();

    let digit_positions =
        generate_unique_positions(opts.digits, total_length, &mut rng, &mut positions);
    let uppercase_positions =
        generate_unique_positions(opts.uppercase, total_length, &mut rng, &mut positions);

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
        .chunks(opts.length)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(&opts.separator)
}

fn generate_unique_positions(
    count: usize,
    max: usize,
    rng: &mut impl Rng,
    used_positions: &mut HashSet<usize>,
) -> HashSet<usize> {
    let mut positions = HashSet::with_capacity(count);

    while positions.len() < count {
        let pos = rng.gen_range(0..max);

        if !used_positions.contains(&pos) {
            positions.insert(pos);
            used_positions.insert(pos);
        }
    }

    positions
}
