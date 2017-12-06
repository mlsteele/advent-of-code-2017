mod wrap;
use wrap::Wrap;

use std::collections::HashSet;

type Result<T> = std::result::Result<T, String>;

type Bank = i64;
type Area = Vec<Bank>;

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let area = parse(&intxt)?;
    let answer = q6p1(area);
    println!("{}", answer);
    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    use std::io::Read;
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(format!("open file ({}): {}", path, err)),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(format!("read file: {}", err));
    }
    Ok(contents)
}

fn parse(s: &str) -> Result<Area> {
    s.trim().lines()
        .next().ok_or_else(|| format!("unexpected lack of input lines"))
        .and_then(|line| {
            let mut res = Vec::new();
            for x in line.trim().split_whitespace() {
                let n: i64 = x.parse().map_err(|_| format!("unexpected non integer: {}", x))?;
                res.push(n);
            }
            Ok(res)
        })
}

// Number of rounds before a dup
fn q6p1(a: Area) -> i64 {
    let mut a = Wrap::new(a);
    let mut seen = HashSet::new();
    for round in 0.. {
        println!("{:?}", a.as_ref());
        if seen.replace(a.as_ref().clone()).is_some() {
            return round
        }
        a.mutate(balancer_round);
    };
    unreachable!();
}

fn balancer_round(mut a: Area) -> Area {
    let sel_i = select_bank(&a);
    let sel_n = a[sel_i];
    a[sel_i] = 0;
    let mut i = sel_i;
    for _ in 0..sel_n {
        i = (i + 1) % a.len();
        a[i] += 1;
    }
    a
}

// Which bank to redistribute from
fn select_bank(a: &Area) -> usize {
    assert!(!a.is_empty());
    let mut best_n = 0;
    let mut best_i = 0;
    for (i, &n) in a.iter().enumerate() {
        assert!(n >= 0);
        if n > best_n {
            best_i = i;
            best_n = n;
        }
    }
    best_i
}
