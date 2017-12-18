use std::collections::{HashMap,HashSet};

type Name = String;
type Shouts = Vec<(Name, i64, Vec<Name>)>;

type Result<T> = std::result::Result<T, String>;

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let shouts = parse(&intxt)?;
    let answer = q7p1(shouts);
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

fn parse(s: &str) -> Result<Shouts> {
    let mut res: Shouts = Vec::new();
    for line in s.trim().lines() {
        let mut words = line.trim().split_whitespace();
        let name = words.next().ok_or_else(|| format!("missing name"))?.to_owned();
        let weight = words.next()
            .ok_or_else(|| format!("missing weight"))?
            .trim_matches('(')
            .trim_matches(')');
        let weight: i64 = weight.parse()
            .map_err(|_| format!("unexpected non integer: {}", weight))?;
        let _arrow = words.next();
        let supporting: Vec<Name> = words
            .map(|x2| x2.trim_matches(',').to_owned()).collect();
        res.push((name, weight, supporting));
    }
    Ok(res)
}

// Number of rounds before a dup
#[allow(dead_code)]
fn q7p1(shouts: Shouts) -> Name {
    let mut all: HashSet<String> = HashSet::new();
    let mut refd: HashSet<String> = HashSet::new();
    for (name, _, supportees) in shouts {
        all.insert(name.to_owned());
        for name in supportees {
            refd.insert(name.to_owned());
        }
    }
    let diff: Vec<&Name> = all.difference(&refd).collect();
    if diff.len() != 1 {
        panic!("unexpected unreferenced node count {}", diff.len())
    }
    (*diff.first().unwrap()).to_owned()
}

#[allow(dead_code)]
fn q7p2(shouts: Shouts) {
    // X supports {Ys}
    let mut supports: HashMap<Name, HashSet<Name>> = HashMap::new();
    // X weighs y
    let mut weight: HashMap<Name, i64> = HashMap::new();
    for (name, weight, supportees) in shouts {
        weight.insert(name, weight);
        for supportee in supportees.iter() {
            merge(supports, name, supportees);
        }
    }

    // X's subtree weights y
    let mut cumweight: HashMap<Name, i64> = HashSet::new();
    while cumweight.len() < shouts.len() {
        for (name, weight, supportees) in shouts {
            let cw = supportees.iter().map(|child| {
                cumweight.get(child)
            }).fold(Some(0), |acc, child_weight| {
                add_opts(acc, child_weight)
            });
            if let Some(cw) = cw {
                cumweight.insert(name, cw);
            }
        }
    }
}

/// Add `val` to the the set at `map[key]`
fn merge<M,K,V>(&mut map: M, key: K, val: V)
    where M: HashMap<K, HashSet<V>>
{
    if let Some(set) = map.get_mut(key) {
        set.insert(val);
        return;
    }
    let mut set = HashSet::new();
    set.insert(val);
    map.insert(key, set);
}

fn add_opts<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.and_then(|x| b.map(|y| x + y))
}
