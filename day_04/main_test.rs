use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const ANSWER1: &str = "Number of XMAS (1): 2591";
const ANSWER2: &str = "Number of X-MAS (2): 1880";

const RESET: &str = "\x1b[0m";

const RED_BACKGROUND: &str = "\x1b[41m";
const GREEN_BACKGROUND: &str = "\x1b[42m";

const RED_BOLD_BRIGHT: &str = "\x1b[1;91m";
const GREEN_BOLD_BRIGHT: &str = "\x1b[1;92m";
const YELLOW_BOLD_BRIGHT: &str = "\x1b[1;93m";
const WHITE_BOLD_BRIGHT: &str = "\x1b[1;97m";

fn print_pass_message() {
    println!("{WHITE_BOLD_BRIGHT}{GREEN_BACKGROUND} PASS {RESET}");
}

fn print_fail_message() {
    println!("{WHITE_BOLD_BRIGHT}{RED_BACKGROUND} FAIL {RESET}");
}

fn print_time(t: f64) {
    println!(
        "{WHITE_BOLD_BRIGHT} Time {} {t:.3} s{RESET}",
        if t < 3.0 {
            GREEN_BOLD_BRIGHT
        } else if t < 20.0 {
            YELLOW_BOLD_BRIGHT
        } else {
            RED_BOLD_BRIGHT
        }
    );
}

fn main() {
    if env::args().len() == 2 && env::args().last().expect("First line must be initial time") == "time" {
        println!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros()
        );
        return;
    }

    let output = fs::read_to_string("output.txt").expect("Should have been able to read the file");
    let mut lines = output.trim().lines();

    let init: u128 = lines
        .next()
        .expect("Should have a line here")
        .parse()
        .expect("Invalid integer");

    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        - init;
    let mut pass = 0;

    if lines.next().expect("Should have a line here") == ANSWER1 {
        pass += 1;
    }

    if lines.next().expect("Should have a line here") == ANSWER2 {
        pass += 1;
    }

    if pass == 2 {
        print_pass_message();
    } else {
        print_fail_message();
    }

    print_time((t as f64) / 1000000.0);
}
