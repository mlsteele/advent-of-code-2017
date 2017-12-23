use std::borrow::Borrow;
use std::collections::{HashSet,HashMap};
use std::hash::Hash;

type Result<T> = std::result::Result<T, String>;

type ID = i64;

#[allow(dead_code)]
fn e<T,S>(msg: S) -> Result<T>
    where S: Borrow<str>
{
    return Err(msg.borrow().to_string());
}

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let intermediate = parse(&intxt)?;
    let answer = q12p1(intermediate)?;
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

fn parse(s: &str) -> Result<Vec<(ID, Vec<ID>)>> {
    let mut res = Vec::new();
    for line in s.trim().lines() {
        let sides: Vec<&str> = line.trim().split("<->").collect();
        if sides.len() != 2 {
            return e(format!("line has bad format: {}", line.trim()));
        }
        let from: ID = sides[0].trim().parse().map_err(|_| "malformed ID")?;
        let mut tos: Vec<ID> = Vec::new();
        for s in sides[1].split(',') {
            let to: ID = s.trim().parse().map_err(|_| "malformed ID")?;
            tos.push(to);
        }
        if tos.len() == 0 {
            return e("no IDs on right side of arrow");
        }
        res.push((from, tos));
    }
    Ok(res)
}

#[allow(dead_code)]
fn q12p1(input: Vec<(ID, Vec<ID>)>) -> Result<i64> {
    println!("input: {:?}", input);
    let mut village = Village::new();
    for (from, tos) in input {
        for to in tos {
            village.connect(from, to);
        }
    }
    println!("pipes: {:?}", village.pipes);
    let ns = village.transitive_neighbors(0);
    println!("transitive neighbors of 0: {:?}", ns);
    Ok(ns.len() as i64)
}

struct Village {
    pipes: HashMap<ID, HashSet<ID>>,
}

impl Village {
    fn new() -> Self { Self{
        pipes: HashMap::new(),
    }}

    fn connect(&mut self, a: ID, b: ID) {
        merge(&mut self.pipes, a, a); // connection to self
        merge(&mut self.pipes, a, b); // forward connection
        merge(&mut self.pipes, b, a); // backward connection
    }

    fn neighbors(&self, of: ID) -> HashSet<ID> {
        self.pipes.get(&of).cloned().unwrap_or_else(|| HashSet::new())
    }

    fn transitive_neighbors(&self, of: ID) -> HashSet<ID> {
        let mut res = HashSet::new();
        res.insert(of);
        loop {
            let mut changed = false;
            for &n1 in res.clone().iter() {
                for &n2 in self.neighbors(n1).iter() {
                    changed |= res.insert(n2);
                }
            }
            if !changed {
                return res;
            }
        }
    }
}

/// Add `val` to the the set at `map[key]`
fn merge<K,V>(map: &mut HashMap<K, HashSet<V>>, key: K, val: V)
    where K: Eq + Hash,
          V: Eq + Hash
{
    if let Some(set) = map.get_mut(&key) {
        set.insert(val);
        return;
    }
    let mut set = HashSet::new();
    set.insert(val);
    map.insert(key, set);
}
