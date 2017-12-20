use std::collections::{HashMap,HashSet};
use std::hash::Hash;

type Name = String;
type Shouts = Vec<(Name, i64, Vec<Name>)>;

type Result<T> = std::result::Result<T, String>;

fn e<T>(msg: &str) -> Result<T> {
    return Err(msg.to_owned());
}

fn main() {
    if let Err(err) = main2() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<()> {
    let intxt = read_file("input.txt")?;
    let shouts = parse(&intxt)?;
    let answer = q7p2(shouts)?;
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

struct Summary {
    pub shouts: Shouts,
    // X weighs Y
    pub weights: HashMap<Name, i64>,
    // X supports [Y]
    pub supports: HashMap<Name, HashSet<Name>>,
    // X's subtree weighs y
    pub stackweights: HashMap<Name, i64>,
}

fn summarize(shouts: Shouts) -> Result<Summary> {
    // X supports {Ys}
    let mut supports: HashMap<Name, HashSet<Name>> = HashMap::new();
    // X weighs y
    let mut weights: HashMap<Name, i64> = HashMap::new();
    for (name, weight, supportees) in shouts.iter().cloned() {
        weights.insert(name.clone(), weight);
        for supportee in supportees.iter() {
            merge(&mut supports, name.clone(), supportee.to_owned());
        }
    }

    // X's subtree weights y
    let mut stackweights: HashMap<Name, i64> = HashMap::new();
    while stackweights.len() < shouts.len() {
        for (name, weight, supportees) in shouts.iter().cloned() {
            let substack_weight = supportees.iter().map(|child| {
                stackweights.get(child)
            }).fold(Some(0), |acc, child_weight| {
                add_opts(acc, child_weight.cloned())
            });
            if let Some(substack_weight) = substack_weight {
                stackweights.insert(name, weight + substack_weight);
            }
        }
    }

    Ok(Summary{
        shouts: shouts,
        weights: weights,
        supports: supports,
        stackweights: stackweights,
    })
}

enum BalancedResult {
    Balanced,
    Unbalanced(Name),
}

use BalancedResult::*;

fn is_balanced(z: &Summary) -> Result<BalancedResult> {
    let mut checked = HashSet::new();
    while checked.len() < z.shouts.len() {
        for (name, _, supportees) in z.shouts.iter().cloned() {
            if checked.contains(&name) {
                // Already checked
                continue
            }
            if supportees.len() == 0 {
                // Leaf node is auto-checked
                checked.insert(name);
                continue
            }
            if !supportees.iter().all(|s| checked.contains(s)) {
                // Not all children have been checked yet
                continue
            }
            let h: Vec<Group<i64,Name>> = group_by(supportees, |s| z.stackweights[s]);
            if h.len() == 1 {
                // Balanced
                checked.insert(name);
                continue
            }
            return Ok(Unbalanced(name));
        }
    }
    Ok(Balanced)
}

#[allow(dead_code)]
fn q7p2(shouts: Shouts) -> Result<i64> {
    let z = summarize(shouts)?;

    for (name, substack_weight) in z.stackweights.iter() {
        println!("stackweight {}: {}", name, substack_weight);
    }

    if let Balanced = is_balanced(&z)? {
        return e("tree is already balanced");
    }

    let mut checked = HashSet::new();
    while checked.len() < z.shouts.len() {
        for (name, _, supportees) in z.shouts.iter().cloned() {
            if checked.contains(&name) {
                // Already checked
                continue
            }
            if supportees.len() == 0 {
                // Leaf node is auto-checked
                checked.insert(name);
                continue
            }
            if !supportees.iter().all(|s| checked.contains(s)) {
                // Not all children have been checked yet
                continue
            }
            let h: Vec<Group<i64,Name>> = group_by(supportees, |s| z.stackweights[s]);
            println!("hist {}: {:?}", name, h);
            if h.len() == 1 {
                // Balanced
                checked.insert(name);
                continue
            }
            println!("unbalanced: {}", name);
            if h.len() != 2 {
                return Err("too many mismatches".to_owned());
            }
            let correctee = &h[0].values[0];
            let delta = h[1].key - h[0].key;
            let new_weight = z.weights[correctee] + delta;
            println!("correcting '{}' to {}", correctee, new_weight);
            return Ok(new_weight);
        }
    }

    return Err("no imbalances detected".to_owned())
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

fn add_opts(a: Option<i64>, b: Option<i64>) -> Option<i64> {
    a.and_then(|x| b.map(|y| x + y))
}

/// Create a histogram of value frequency frequency.
/// Returns a list of number of grouped items, sorted by ascending count.
#[allow(dead_code)]
fn histogram<T,I>(items: I) -> Vec<(T, usize)>
    where T: Eq + Hash + Clone,
          I: IntoIterator<Item=T>
{
    let counts = items.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let mut res: Vec<(T, usize)> = counts.iter().map(|(v, c)| (v.clone(), c.clone())).collect();
    res.sort_by_key(|&(_, count)| count);
    res
}

#[derive(Debug)]
struct Group<K,V> {
    pub key: K,
    pub values: Vec<V>
}

impl<K,V> Group<K,V> {
    fn new(k: K) -> Self {
        Self {
            key: k,
            values: Vec::new(),
        }
    }
}

/// Group items by key(value). Return groups sorted by ascending count.
fn group_by<K,V,I,F>(items: I, mut key: F) -> Vec<Group<K,V>>
    where K: Eq + Hash + Clone,
          I: IntoIterator<Item=V>,
          F: FnMut(&V) -> K
{
    let mut map = items.into_iter().fold(HashMap::new(), |mut acc, v| {
        let k = key(&v);
        acc.entry(k.clone()).or_insert_with(|| Group::new(k)).values.push(v);
        acc
    });
    let mut res: Vec<Group<_,_>> = map.drain().map(|(_, g)| g).collect();
    res.sort_by_key(|xs| xs.values.len());
    res
}
