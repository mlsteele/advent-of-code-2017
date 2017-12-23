use std::collections::vec_deque::VecDeque;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, String>;

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
    let answer = q11p2(intermediate)?;
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

#[derive(Debug)]
enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Dir {
    fn parse(s: &str) -> Result<Self> {
        use Dir::*;
        match s {
            "n" => Ok(N),
            "nw" => Ok(NW),
            "ne" => Ok(NE),
            "sw" => Ok(SW),
            "se" => Ok(SE),
            "s" => Ok(S),
            s => e(format!("unrecognized direction: {}", s))
        }
    }

    fn unit(&self) -> Pos {
        use Dir::*;
        match *self {
            N  => Pos::at(1, 0),
            NE => Pos::at(0, 1),
            SE => Pos::at(-1, 1),
            S  => Pos::at(-1, 0),
            SW => Pos::at(0, -1),
            NW => Pos::at(1, -1),
        }
    }

    fn all() -> Vec<Dir> {
        use Dir::*;
        vec![N, NE, SE, S, SW, NW]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: i64, // N-S
    pub y: i64, // NE-SW
}

impl Pos {
    fn origin() -> Self { Self{x: 0, y: 0} }

    fn at(x: i64, y: i64) -> Self { Self{x: x, y: y} }

    fn neighbors(&self) -> Vec<Self> {
        let s = self;
        Dir::all().iter().map(|d| *s + d.unit()).collect()
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse(s: &str) -> Result<Vec<Dir>> {
    let mut res = Vec::new();
    for x in s.trim().lines()
        .next().ok_or_else(|| "no input line")?
        .split(',')
    {
        res.push(Dir::parse(x.trim())?)
    }
    Ok(res)
}

#[allow(dead_code)]
fn q11p1(path: Vec<Dir>) -> Result<i64> {
    println!("path: {:?}", path);
    let start = Pos::origin();
    let mut p = start;
    for (i,d) in path.iter().enumerate() {
        p += Dir::unit(d);
        println!("{} {:?}: {:?}", i, d, p);
    }
    for y in Bloom::new(start) {
        if p == y.pos {
            return Ok(y.distance as i64)
        }
    }
    return e("unreachable end of search")
}

#[allow(dead_code)]
fn q11p2(path_dirs: Vec<Dir>) -> Result<i64> {
    println!("path: {:?}", path_dirs);
    let start = Pos::origin();
    let mut path = vec![start];
    for (i,d) in path_dirs.iter().enumerate() {
        let p = *path.last().unwrap() + Dir::unit(d);
        println!("{} {:?}: {:?}", i, d, p);
        path.push(p);
    }

    let mut search = Bloom::new(start);
    let mut path_dists = vec![0];
    let mut distmap: HashMap<Pos, usize> = HashMap::new();
    for (i,p) in path.iter().enumerate() {
        println!("searching for [{}]: {:?}", i, p);
        'inner: loop {
            if let Some(&dist) = distmap.get(p) {
                path_dists.push(dist);
                break 'inner;
            }
            if let Some(item) = search.next() {
                distmap.insert(item.pos, item.distance);
            } else {
                return e("unreachable end of search")
            }
        }
    }

    Ok(*path_dists.iter().max().unwrap() as i64)
}

#[derive(Debug)]
struct BloomItem {
    pos: Pos,
    distance: usize,
}

struct Bloom {
    queue: VecDeque<BloomItem>,
    visited: HashSet<Pos>,
}

impl Bloom {
    fn new(start: Pos) -> Self {
        Self {
            queue: VecDeque::from(vec![BloomItem{pos: start, distance: 0}]),
            visited: HashSet::new(),
        }
    }
}

impl Iterator for Bloom {
    type Item = BloomItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.queue.pop_front() {
            if self.visited.insert(item.pos) {
                // Not yet visited p.
                let nexts: Vec<BloomItem> = item.pos.neighbors().iter().map(|n| {
                    BloomItem{pos: *n, distance: item.distance + 1}
                }).collect();
                self.queue.extend(nexts);
                return Some(item);
            }
        }
        None // probably should never happen
    }
}
