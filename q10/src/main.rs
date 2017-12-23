use std::borrow::Borrow;
use std::fmt;

type Result<T> = std::result::Result<T, String>;

#[allow(dead_code)]
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
    let instructions = parse(&intxt)?;
    let answer = q10p1(instructions)?;
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

fn parsep1(s: &str) -> Result<Vec<i64>> {
    let mut res = Vec::new();
    for x in s.trim().lines()
        .next().ok_or_else(|| "no input line")?
        .split(',')
    {
        res.push(x.trim().parse().map_err(|_| "unexpected non-integer")?);
    }
    Ok(res)
}

#[allow(dead_code)]
fn q10p1(lengths: Vec<i64>) -> Result<i64> {
    println!("{:?}", lengths);
    let mut s = State::new(256);
    println!("{:?}", s);
    for &l in lengths.iter() {
        step(&mut s, l)?;
        println!("{} -> {:?}", l, s);
    }
    Ok(s.list[0] * s.list[1])
}

struct State {
    list: Vec<i64>,
    pos: i64,
    skip: i64,
}

impl State {
    fn new(len: i64) -> Self { Self{
        list: std::ops::Range{start: 0, end: len}.collect(),
        pos: 0,
        skip: 0,
    }}
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, e) in self.list.iter().enumerate() {
            if i == self.pos as usize {
                write!(f, "[{:?}]", e)?;
            } else {
                write!(f, "{:?}", e)?;
            }
            if i+1 < self.list.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, " skip:{}", self.skip)
    }
}

fn step(s: &mut State, length: i64) -> Result<()> {
    s.list = reverse_cyclic(&s.list, s.pos as usize, length as usize);
    s.pos += length + s.skip;
    s.pos %= s.list.len() as i64;
    s.skip += 1;
    s.skip %= s.list.len() as i64;
    Ok(())
}

// Reverse a subsection of a vec. Wrapping.
fn reverse_cyclic<T,V>(v: V, start: usize, len: usize) -> Vec<T>
    where V: Borrow<Vec<T>>,
          T: Clone
{
    let v = v.borrow();
    let start = start % v.len();
    let it1 = v.iter().cycle();
    let it2 = v.iter().cycle();
    let segment: Vec<T> = it1.skip(start).take(len).cloned().collect();
    segment.iter().rev()
        .chain(it2.skip(start+len).take(v.len()-len))
        .cycle()
        .skip(v.len()-start)
        .take(v.len())
        .cloned().collect()
}

#[test]
fn it_works() {
    assert_eq!(vec![4,3,2,1,0], reverse_cyclic(vec![0,1,2,3,4], 3, 4));
}
