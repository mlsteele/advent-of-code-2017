use std::fmt;
use std::collections::{HashMap};

type Result<T> = std::result::Result<T, String>;

type RegisterName = String;

#[derive(Debug)]
enum Op {
    Inc,
    Dec,
}

use Op::*;

// >, <, >=, <=, ==, !=
#[derive(Debug)]
enum Comparator {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Neq,
}

use Comparator::*;

#[derive(Debug)]
struct Condition {
    source: RegisterName,
    comparator: Comparator,
    operand: i64,
}

#[derive(Debug)]
struct Instruction {
    pub target: RegisterName,
    pub op: Op,
    pub operand: i64,
    pub condition: Condition,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {} if {} {:?} {}",
               self.target,
               self.op,
               self.operand,
               self.condition.source,
               self.condition.comparator,
               self.condition.operand)
    }
}

struct State {
    pub registers: HashMap<RegisterName, i64>,
}

impl State {
    fn new() -> Self {
        Self{
            registers: HashMap::new(),
        }
    }

    fn get(&self, r: &RegisterName) -> i64 {
        self.registers.get(r).map(|x| *x).unwrap_or(0)
    }

    fn set(&mut self, r: &RegisterName, val: i64) {
        self.registers.insert(r.clone(), val);
    }
}

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
    let answer = q8p2(instructions)?;
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

fn parse(s: &str) -> Result<Vec<Instruction>> {
    let mut res = Vec::new();
    for line in s.trim().lines() {
        let mut words = line.trim().split_whitespace();
        let target = words.next().ok_or_else(|| format!("missing name"))?.to_owned();
        let op: Op = match words.next().ok_or_else(|| format!("missing weight"))? {
            "inc" => Inc,
            "dec" => Dec,
            s => return e(&format!("unrecognized op: {}", s)),
        };
        let operand: i64 = words.next()
            .ok_or_else(|| format!("missing operand"))?
            .parse()
            .map_err(|_| "unexpected non-integer: {}")?;
        let _if = words.next();
        let condition = Condition{
            source: words.next()
                .ok_or_else(|| "missing condition source")?.to_owned(),
            comparator: match words.next().ok_or_else(|| "missing comparator")? {
                ">" => Gt,
                "<" => Lt,
                ">=" => Ge,
                "<=" => Le,
                "==" => Eq,
                "!=" => Neq,
                s => return e(&format!("unrecognized comparator: {}", s)),
            },
            operand: words.next()
                .ok_or_else(|| "missing comparator operand")?
                .parse()
                .map_err(|_| "unexpected non-integer")?,
        };
        res.push(Instruction{
            target: target,
            op: op,
            operand: operand,
            condition: condition,
        });
    }
    Ok(res)
}

// Maximum register value at the end.
#[allow(dead_code)]
fn q8p1(instructions: Vec<Instruction>) -> Result<i64> {
    for instruction in instructions.iter() {
        println!("{}", instruction)
    }
    let state = simulate(instructions);
    state.registers.values().max()
        .map(|x| *x).ok_or_else(|| "no registers".to_owned())
}

// Maximum register value at any point.
#[allow(dead_code)]
fn q8p2(instructions: Vec<Instruction>) -> Result<i64> {
    let mut max = 0;
    let mut state = State::new();
    for ins in instructions.iter() {
        step(&mut state, ins);
        if let Some(m) = state.registers.values().max() {
            if *m > max {
                max = *m
            }
        }
    }
    Ok(max)
}

fn simulate(instructions: Vec<Instruction>) -> State {
    instructions.iter().fold(State::new(), |mut state, ins| {
        step(&mut state, &ins);
        state
    })
}

fn step(state: &mut State, ins: &Instruction) {
    let src = state.get(&ins.condition.source);
    if comparate(src, &ins.condition.comparator, ins.condition.operand) {
        let res = operate(state.get(&ins.target), &ins.op, ins.operand);
        state.set(&ins.target, res);
    }
}

fn operate(a: i64, op: &Op, b: i64) -> i64 {
    match *op {
        Inc => a + b,
        Dec => a - b,
    }
}

fn comparate(a: i64, comparator: &Comparator, b: i64) -> bool {
    match *comparator {
        Gt => a > b,
        Lt => a < b,
        Ge => a >= b,
        Le => a <= b,
        Eq => a == b,
        Neq => a != b,
    }
}
