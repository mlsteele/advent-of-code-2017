use std::collections::HashSet;

type Result<T> = std::result::Result<T, String>;

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let phrases = parse(&intxt);
    let answer = q4p2(phrases);
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

fn parse(s: &str) -> Vec<Vec<String>> {
    s.trim().lines().map(|l| {
        l.trim()
    }).filter(|l| {
        !l.is_empty()
    }).map(|l| {
        l.split_whitespace().map(|p| p.to_owned()).collect()
    }).collect()
}

// Number of valid passphrases.
#[allow(dead_code)]
fn q4p1(lines: Vec<Vec<String>>) -> i64 {
    lines.iter().filter(|ref l| is_valid_p1(*l)).count() as i64
}

// Number of valid passphrases under the new draconian policy.
fn q4p2(lines: Vec<Vec<String>>) -> i64 {
    lines.iter().filter(|ref l| is_valid_p2(*l)).count() as i64
}

#[allow(dead_code)]
fn is_valid_p1(phrase: &Vec<String>) -> bool {
    let mut seen = HashSet::new();
    for word in phrase.iter() {
        if seen.replace(word).is_some() {
            return false
        }
    }
    return true
}

fn is_valid_p2(phrase: &Vec<String>) -> bool {
    let mut seen = HashSet::new();
    for word in phrase.iter() {
        let mut cs: Vec<char> = word.chars().collect();
        cs.sort();
        if seen.replace(cs).is_some() {
            return false
        }
    }
    return true
}
