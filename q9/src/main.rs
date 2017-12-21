use std::fmt;
use std::borrow::Borrow;

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
    let answer = q9p2(instructions)?;
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

struct Group {
    items: Vec<Box<LI>>,
}

impl Group {
    fn new() -> Self { Self{ items: Vec::new() } }

    fn push_group(&mut self, g: Group) {
        self.items.push(Box::new(LI::Group(g)));
    }

    fn push_junk(&mut self, j: Junk) {
        self.items.push(Box::new(LI::Junk(j)));
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "G{:?}", self.items)
    }
}

struct Junk {
    // number of characters in this junk
    count: i64,
}

impl Junk {
    fn new() -> Self { Self{ count: 0 } }
}

impl fmt::Debug for Junk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "J{}", self.count)
    }
}

enum LI {
    Group(Group),
    Junk(Junk),
}

impl fmt::Debug for LI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LI::Group(ref g) => write!(f, "{:?}", g),
            &LI::Junk(ref j) => write!(f, "{:?}", j),
        }
    }
}

#[derive(Debug)]
struct Parser {
    top: Option<LI>,
    stack: Vec<Group>,
    cancel: bool,
    done: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self{
            top: None,
            stack: Vec::new(),
            cancel: false,
            done: false,
        }
    }

    pub fn push(&mut self, c: char) -> Result<()> {
        if c == ' ' {
            // skip spaces
            return Ok(());
        }
        if let Some(top) = self.top.take() {
            match top {
                LI::Group(g) => self.push_in_group(c, g),
                LI::Junk(j)  => self.push_in_junk(c, j),
            }
        } else {
            self.push_empty(c)
        }
    }

    pub fn finish(self) -> Result<LI> {
        if !self.done {
            return e("not done");
        }
        let n = self.stack.len();
        if n > 0 {
            return e(&format!("finished with {} open groups", n));
        }
        if let Some(top) = self.top {
            Ok(top)
        } else {
            e("no parsed items")
        }
    }

    fn push_empty(&mut self, c: char) -> Result<()> {
        match c {
            '{' => {
                self.top = Some(LI::Group(Group::new()));
            },
            '<' => {
                self.top = Some(LI::Junk(Junk::new()));
            },
            c => return e(&format!("unexpected '{}'", c))
        };
        Ok(())
    }

    fn push_in_group(&mut self, c: char, g: Group) -> Result<()> {
        match c {
            '{' => {
                self.stack.push(g);
                self.top = Some(LI::Group(Group::new()));
            },
            '}' => {
                // g has ended
                if let Some(mut parent) = self.stack.pop() {
                    parent.push_group(g);
                    self.top = Some(LI::Group(parent));
                } else {
                    self.top = Some(LI::Group(g));
                    if self.done {
                        return e("closed group but no groups open")
                    }
                    self.done = true
                }
            },
            '<' => {
                self.stack.push(g);
                self.top = Some(LI::Junk(Junk::new()));
            },
            ',' => {
                // Ignore commas. Too lazy to make sure they're correct.
                self.top = Some(LI::Group(g));
            }
            c => return e(&format!("unexpected character: {}", c))
            // {{<ab>},{},{<ab>,{<!!>}},{<ab>}}
        };
        Ok(())
    }

    fn push_in_junk(&mut self, c: char, mut j: Junk) -> Result<()> {
        if self.cancel {
            // drop the canceled char
            self.cancel = false;
            self.top = Some(LI::Junk(j));
            return Ok(());
        }
        match c {
            '!' => {
                self.cancel = true;
                self.top = Some(LI::Junk(j));
            },
            '>' => {
                // j has ended
                if let Some(mut parent) = self.stack.pop() {
                    parent.push_junk(j);
                    self.top = Some(LI::Group(parent));
                } else {
                    return e("junk ended outside of group");
                }
            }
            _ => {
                j.count += 1;
                self.top = Some(LI::Junk(j));
            }
        };
        Ok(())
    }
}

fn parse(s: &str) -> Result<Group> {
    let mut p = Parser::new();
    for c in s.trim().chars() {
        p.push(c)?;
        // println!("{} {:?}", c, p);
    }
    p.finish().and_then(|x| match x {
        LI::Group(g) => Ok(g),
        LI::Junk(_) => e("expected group but parsed junk")
    })
}

// Maximum register value at the end.
#[allow(dead_code)]
fn q9p1(g: Group) -> Result<i64> {
    println!("{:?}", g);
    Ok(score(&g, 1))
}

fn q9p2(g: Group) -> Result<i64> {
    println!("{:?}", g);
    Ok(countjunk(&LI::Group(g)))
}

fn score(g: &Group, depth: i64) -> i64 {
    let sub: i64 = g.items.iter().filter_map(|g2| match **g2 {
        LI::Group(ref g2) => Some(score(g2, depth+1)),
        LI::Junk(_) => None,
    }).sum();
    depth + sub
}

fn countjunk<T>(item: &T) -> i64
    where T: Borrow<LI>
{
    match item.borrow() {
        &LI::Group(ref g) => g.items.iter().map(countjunk).sum(),
        &LI::Junk(ref j) => j.count,
    }
}

