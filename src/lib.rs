use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum Op {
    Num(f64),
    Add,
    Sub,
    Div,
    Mul,
    Pow,
    Swp,
    Dup,
    Drp,
    Log,
    Get,
    Set,
}

impl<'a> TryFrom<&'a str> for Op {
    type Error = &'a str;

    // Required method
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "x" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            "%" => Ok(Self::Swp),
            "#" => Ok(Self::Dup),
            "_" => Ok(Self::Drp),
            "log" => Ok(Self::Log),
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            x => match x.parse::<f64>() {
                Ok(n) => Ok(Self::Num(n)),
                Err(_) => Err(value),
            },
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Op::Num(n) => write!(f, "{}", n),
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Pow => write!(f, "^"),
            Op::Swp => write!(f, "%"),
            Op::Dup => write!(f, "#"),
            Op::Drp => write!(f, "_"),
            Op::Log => write!(f, "log"),
            Op::Get => write!(f, "get"),
            Op::Set => write!(f, "set"),
        }
    }
}

#[derive(Debug)]
pub struct Env {
    pub stack: Vec<Op>,
    pub regs: HashMap<u8, Op>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            regs: HashMap::new(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Env {
    type Error = &'a str;

    // Required method
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut stack = vec![];

        for word in value.split_whitespace() {
            let op = Op::try_from(word)?;
            stack.push(op);
        }

        Ok(Self {
            stack,
            regs: HashMap::new(),
        })
    }
}

#[test]
fn make_env() {
    let env = Env::try_from("1 2 + 3 * 2 ^").unwrap();
    insta::assert_snapshot!(env);
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut reg_keys = self.regs.keys().collect::<Vec<_>>();
        reg_keys.sort_unstable();

        for key in reg_keys.iter() {
            let val = self.regs.get(key).unwrap();
            write!(f, "({}: {}) ", key, val)?;
        }

        for (i, val) in self.stack.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{:>5}", val)?;
        }

        Ok(())
    }
}

impl Env {
    pub fn push(&mut self, op: Op) {
        self.stack.push(op);
    }

    pub fn eval(&mut self) -> Option<bool> {
        let stack = &mut self.stack;
        let op = stack.pop()?;

        match op {
            Op::Drp => {
                let _ = stack.pop();
                Some(true)
            }

            Op::Num(n) => {
                stack.push(Op::Num(n));
                Some(false)
            }

            Op::Add => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a + b));
                    Some(true)
                }
                _ => None,
            },

            Op::Sub => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(b - a));
                    Some(true)
                }
                _ => None,
            },

            Op::Div => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(b / a));
                    Some(true)
                }
                _ => None,
            },

            Op::Mul => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a * b));
                    Some(true)
                }
                _ => None,
            },

            Op::Pow => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a.powf(b)));
                    Some(true)
                }
                _ => None,
            },

            Op::Log => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(b.log(a)));
                    Some(true)
                }
                _ => None,
            },

            Op::Swp => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a));
                    stack.push(Op::Num(b));
                    Some(true)
                }
                _ => None,
            },

            Op::Dup => match stack.pop()? {
                Op::Num(a) => {
                    stack.push(Op::Num(a));
                    stack.push(Op::Num(a));
                    Some(true)
                }
                _ => None,
            },

            Op::Set => match (stack.pop()?, stack.pop()?) {
                (Op::Num(key), val) => {
                    let key = key as u8;
                    let _ = self.regs.insert(key, val);
                    Some(true)
                }
                _ => None,
            },

            Op::Get => match stack.pop()? {
                Op::Num(key) => {
                    let key = key as u8;
                    let val = self.regs.remove(&key)?;
                    stack.push(val);
                    Some(true)
                }
                _ => None,
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.regs.len() + self.stack.len() == 0
    }

    pub fn fill_from<Iter: IntoIterator<Item = Op>>(&mut self, iter: Iter) {
        for op in iter.into_iter() {
            self.stack.push(op)
        }
    }
}

impl FromIterator<Op> for Env {
    fn from_iter<T: IntoIterator<Item = Op>>(iter: T) -> Self {
        let stack = iter.into_iter().collect();
        Self {
            regs: HashMap::new(),
            stack,
        }
    }
}
