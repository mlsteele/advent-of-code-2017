type Result<T> = std::result::Result<T, String>;

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

    let digits: Vec<i64> = parse(&instr)?;
    let answer = q1(digits);
    println!("{}", answer);
    Ok(())
}

fn parse(s: &str) -> Result<Vec<i64>> {
    let mut res = vec!();
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            let x: i64 = match c.to_string().parse() {
                Ok(x) => x,
                Err(err) => return Err(format!("parse error: {}", err)),
            };
            res.push(x);
        } else {
            return Err(format!("'{}' is not a digit", c).to_owned());
        }
    }
    return Ok(res);
}

fn q1(s: Vec<i64>) -> i64 {
    if s.is_empty() {
        return 0
    }
    let first: i64 = s[0];
    let z1 = s.iter();
    let z2 = s.iter().skip(1).chain(std::iter::once(&first));
    z1.zip(z2).filter_map(|(x, y)| {
        if x == y {
            Some(x)
        } else {
            None
        }
    }).sum()
}
