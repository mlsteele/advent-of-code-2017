use std::collections::{HashMap,HashSet};
use std::hash::Hash;

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

#[allow(dead_code)]
fn q7p2(shouts: Shouts) -> Result<i64> {
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

    for (name, substack_weight) in stackweights.iter() {
        println!("stackweight {}: {}", name, substack_weight);
    }

    // for x in stackweights.iter() {
    //     println!("{:?}", x);
    // }

    let mut checked = HashSet::new();
    while checked.len() < shouts.len() {
        for (name, _, supportees) in shouts.iter().cloned() {
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
            let h: Vec<(i64, usize)> = histogram(supportees.iter().map(|s| stackweights[s]));
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
            let delta = h[1].0 - h[0].0;
            return Ok();
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

/// Create a histogram of frequency.
/// Returns a list of number of occurrences, sorted by ascending count.
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
