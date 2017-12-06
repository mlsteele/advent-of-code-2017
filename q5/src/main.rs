type Result<T> = std::result::Result<T, String>;

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let phrases = parse(&intxt)?;
    let answer = q5p1(phrases);
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

fn parse(s: &str) -> Result<Vec<i64>> {
    let lines = s.trim().lines().map(|l| {
        l.trim()
    }).filter(|l| {
        !l.is_empty()
    });
    let mut res = Vec::new();
    for line in lines {
        let n: i64 = line.parse().map_err(|_| format!("unexpected non integer: {}", line))?;
        res.push(n);
    }
    Ok(res)
}

// Number of valid passphrases under the new draconian policy.
fn q5p1(mut tape: Vec<i64>) -> i64 {
    let mut pc: i64 = 0;
    let mut tick = 0;
    while pc >= 0 && pc < tape.len() as i64 {
        tick += 1;
        let jump = tape[pc as usize];
        let change = if jump >= 3 {
            -1
        } else {
            1
        };
        tape[pc as usize] += change;
        pc += jump;
    }
    tick
}
