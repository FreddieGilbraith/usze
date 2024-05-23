use atty::Stream;
use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead},
};

#[derive(Debug)]
enum Op {
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
        }
    }
}

impl Op {
    fn print_stack(op: Op, stack: &[Op]) -> () {
        eprint!("{:>5} | ", op);

        for op in stack {
            eprint!("{} ", op);
        }

        eprintln!("");
    }

    fn eval(stack: &mut Vec<Op>, op: Op) -> Option<Op> {
        match op {
            Op::Drp => stack.pop(),

            Op::Num(n) => {
                stack.push(Op::Num(n));
                Some(Op::Num(n))
            }

            Op::Add => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a + b));
                    Some(op)
                }
                _ => None,
            },

            Op::Sub => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(b - a));
                    Some(op)
                }
                _ => None,
            },

            Op::Div => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(b / a));
                    Some(op)
                }
                _ => None,
            },

            Op::Mul => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a * b));
                    Some(op)
                }
                _ => None,
            },

            Op::Pow => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a.powf(b)));
                    Some(op)
                }
                _ => None,
            },

            Op::Log => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a.log(b)));
                    Some(op)
                }
                _ => None,
            },

            Op::Swp => match (stack.pop()?, stack.pop()?) {
                (Op::Num(a), Op::Num(b)) => {
                    stack.push(Op::Num(a));
                    stack.push(Op::Num(b));
                    Some(op)
                }
                _ => None,
            },

            Op::Dup => match stack.pop()? {
                Op::Num(a) => {
                    stack.push(Op::Num(a));
                    stack.push(Op::Num(a));
                    Some(op)
                }
                _ => None,
            },
        }
    }
}

fn eval_args(stack: &mut Vec<Op>, verbose: bool) -> Result<(), Box<dyn Error>> {
    for arg in env::args().skip(1) {
        for word in arg.split_whitespace() {
            let op = Op::try_from(word)?;

            if let Some(val) = Op::eval(stack, op) {
                if verbose {
                    Op::print_stack(val, &stack);
                }
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn eval_stdin(stack: &mut Vec<Op>, verbose: bool) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    'outer: for line in stdin.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                let op = Op::try_from(word)?;

                if let Some(val) = Op::eval(stack, op) {
                    if verbose {
                        Op::print_stack(val, &stack);
                    }
                } else {
                    break 'outer;
                }

                if stack.len() == 1 {
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}

// fn main() {
//     // Create an atomic flag to detect the Ctrl+C signal
//     let running = Arc::new(AtomicBool::new(true));
//     let r = running.clone();

//     // Set up the Ctrl+C handler
//     ctrlc::set_handler(move || {
//         r.store(false, Ordering::SeqCst);
//         println!("Ctrl+C detected!");
//     }).expect("Error setting Ctrl-C handler");

//     // Main loop
//     while running.load(Ordering::SeqCst) {
//         // Simulate work
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         println!("Running...");
//     }

//     println!("Exiting gracefully.");
// }

fn main() -> Result<(), Box<dyn Error>> {
    let interactive = atty::is(Stream::Stdin);
    let verbose = interactive || atty::is(atty::Stream::Stdout);

    let mut stack = vec![];

    if interactive {
        eval_args(&mut stack, verbose)?;
        loop {
            eval_stdin(&mut stack, verbose)?;
        }
    } else {
        eval_stdin(&mut stack, verbose)?;
        eval_args(&mut stack, verbose)?;
    }

    if stack.len() == 1 {
        println!("{}", stack[0]);
    }

    Ok(())
}
