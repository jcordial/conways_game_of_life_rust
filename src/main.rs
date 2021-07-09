use game_of_life::Life;
use rand::Rng;
use std::{thread, time};

use clap::{AppSettings, Clap};
use std::process::exit;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    width: usize,
    height: usize,
}
fn main() {
    let opts: Opts = Opts::parse();
    let life = Life::new(opts.width, opts.height);
    let mut seed: String = String::new();
    let area = opts.width * opts.height;
    for index in 0..(area) {
        if index > 0 && index % opts.width == 0 {
            seed.push_str("\n");
        }

        if rand::thread_rng().gen_range(0..=1) == 0 {
            seed.push_str(" ");
        } else {
            seed.push_str("█")
        }
    }
    println!("{}", seed);
    let mut next = seed;

    for _ in 0..1000 {
        let ten_millis = time::Duration::from_millis(20);

        thread::sleep(ten_millis);
        let prev = next.clone();
        next = life.tick(&next);
        println!("{}", next);
        if !next.contains("█") {
            println!("All cells dead. Exiting.");
            exit(0);
        } else if *prev == next {
            println!("Life has grown predictable, ending the world.");
            exit(0);
        }
        for i in 0..opts.height {
            print!("\x1B[{}A", 1);
        }
    }
    println!("1000 generations created, destroying the world.")
}
