use std::io::{self, Write};
use std::process::{self, Command};
use std::{env, thread, time};

const BEEP_SLEEP_MS: u64 = 100;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: done-beep <command and args>");
        process::exit(1);
    }

    let mut cmd = Command::new(&args[1]);
    if args.len() > 1 {
        args.drain(0..2);
        cmd.args(args);
    }

    match execute(&mut cmd) {
        Some(code) => process::exit(code),
        None => process::exit(1),
    }
}

fn execute(cmd: &mut Command) -> Option<i32> {
    let out = match cmd.output() {
        Ok(out) => {
            io::stdout().write_all(&out.stdout).unwrap();
            io::stderr().write_all(&out.stderr).unwrap();
            out
        }
        Err(err) => {
            eprintln!("Failed to execute process: {err}");
            process::exit(1);
        }
    };

    beep();

    out.status.code()
}

fn beep() {
    for _ in 1..3 {
        println!("\x07\x07");
        thread::sleep(time::Duration::from_millis(BEEP_SLEEP_MS));
    }
}
