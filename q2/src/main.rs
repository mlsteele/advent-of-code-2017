type Result<T> = std::result::Result<T, String>;

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let sheet = parse(&intxt)?;
    let answer = q2p1(sheet);
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

fn parse(s: &str) -> Result<Vec<Vec<i64>>> {
    let mut sheet: Vec<Vec<i64>> = Vec::new();
    for line in s.trim().lines() {
        let mut linev: Vec<i64> = Vec::new();
        for cell in line.trim().split_whitespace() {
            let x: i64 = match cell.parse() {
                Ok(x) => x,
                Err(err) => return Err(format!("parse int: {}", err)),
            };
            linev.push(x);
        }
        sheet.push(linev);
    }
    Ok(sheet)
}

fn q2p1(sheet: Vec<Vec<i64>>) -> i64 {
    sheet.iter().map(|line| {
        if !line.is_empty() {
            line.iter().max().unwrap() - line.iter().min().unwrap()
        } else {
            0
        }
    }).sum()
}
