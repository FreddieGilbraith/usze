use atty::Stream;
use std::{
    env,
    error::Error,
    io::{self, BufRead, Write},
};

use calc::*;

fn eval_args(stack: &mut Env, verbose: bool) -> Result<(), Box<dyn Error>> {
    for arg in env::args().skip(1) {
        for word in arg.split_whitespace() {
            let op = Op::try_from(word)?;

            let _ = stack.eval(op);

            if verbose {
                print!("{} ", stack);
                io::stdout().flush()?;
            }
        }
    }

    Ok(())
}

fn eval_stdin(stack: &mut Env, verbose: bool) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    'outer: for line in stdin.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                let op = Op::try_from(word)?;

                let _ = stack.eval(op);

                if verbose {
                    print!("{} ", stack);
                    io::stdout().flush()?;
                }

                if stack.stack.len() == 1 {
                    break 'outer;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let interactive = atty::is(Stream::Stdin);
    let verbose = interactive || atty::is(atty::Stream::Stdout);

    let mut stack = Env::new();

    if interactive {
        eval_args(&mut stack, verbose)?;
        loop {
            eval_stdin(&mut stack, verbose)?;
        }
    } else {
        eval_stdin(&mut stack, verbose)?;
        eval_args(&mut stack, verbose)?;
    }

    if stack.stack.len() == 1 {
        println!("{}", stack.stack[0]);
    }

    Ok(())
}
