use clap::Parser;

// ranges:
//      33-46  => {! ... .}
//      48-57  => {0 ... 9}
//      58-64  => {; ... @}
//      65-90  => {A ... Z}
//      97-122 => {a ... z}

//                             <-special chars->   <Numbers> <small and cap chars>
const RANGES: [(u8, u8); 5] = [(33, 46), (58, 64), (48, 57), (65, 90), (97, 122)];

use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 15)]
    length: u32,

    /// wether to enable special characters
    #[arg(short, long)]
    spec_chars: bool,
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    let mut pass = String::new();
    let start: usize = if args.spec_chars { 0 } else { 2 };
    for _ in 0..args.length {
        let a = rng.gen_range(start..RANGES.len());
        let cur_range = RANGES[a];
        let b = rng.gen_range(cur_range.0..=cur_range.1) as char;
        pass.push(b);
    }
    println!("{pass}");
}
