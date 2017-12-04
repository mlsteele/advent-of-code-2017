type Result<T> = std::result::Result<T, String>;

// How many squares in this layer border
// layer(0) -> 1
// layer(1) -> 8
// layer(2) -> 16

// How long is each side of this layer
// side(0) -> 1
// side(1) -> 3
// side(2) -> 5

// side(n) -> 1 + 2n

// layer(n+1) -> 4 + 4 * side(n)

// layer 0: 0
// layer 1: 1 2 1 2 1 2 1 2
// layer 2: 3 2 3 4 3 2 3 4 3 2 3 4 3 2 3 4
// layer 3: 5 4 3 4 5 6 5 4 3 4 5 6 5 4 3 ...

// mindist(n) -> n
// maxdist(n) -> 2n

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let instr = match args.next() {
        Some(x) => x,
        None => return Err(format!("Usage: program <input>")),
    };

    let x: i64 = parse(&instr)?;
    let answer = q3p1(x);
    println!("{}", answer);
    Ok(())
}

fn parse(s: &str) -> Result<i64> {
    let x: i64 = match s.parse() {
        Ok(x) => x,
        Err(err) => return Err(format!("parse int: {}", err)),
    }
}

// Length of a size of the nth layer.
fn side(n: i64) -> i64 {
    1 + 2 * n
}

// How many squares in this layer border
fn layer(n: i64) -> i64 {
    4 + 4 * side(n-1)
}

// Minimum distance to the center from layer n.
fn mindist(n: i64) -> i64 { n }

// Maximum distance to the center from layer n.
fn mindist(n: i64) -> i64 { 2 * n }

fn q3p1(x: i64) -> i64 {
    let mut consumed = 0;
    for n in 0.. {
        consumed += layer(n);
    }
}
