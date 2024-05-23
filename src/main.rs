use atty::Stream;
use std::{
    env,
    error::Error,
    io::{self, BufRead, Write},
};

use usze::*;

fn eval_and_print_till_terminal(env: &mut Env) -> Result<(), Box<dyn Error>> {
    print!("{} ", env);
    io::stdout().flush()?;
    while let Some(true) = env.eval() {
        print!("\n{} ", env);
        io::stdout().flush()?;
    }
    Ok(())
}

fn eval_stdin(env: &mut Env, verbose: bool) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();

    for line in stdin.lines() {
        if let Ok(line) = line {
            for (i, word) in line.split_whitespace().enumerate() {
                let op = Op::try_from(word)?;
                if verbose && i != 0 {
                    if i == 1 {
                        println!("");
                    }
                    println!("{}", env);
                }
                env.push(op);
            }

            eval_and_print_till_terminal(env)?;
        }
    }

    Ok(())
}

fn push_from_args(env: &mut Env) -> Result<(), Box<dyn Error>> {
    for arg in env::args().skip(1) {
        for word in arg.split_whitespace() {
            let op = Op::try_from(word)?;
            env.push(op)
        }
    }

    Ok(())
}

fn push_from_stdin(env: &mut Env) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();

    for line in stdin.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                let op = Op::try_from(word)?;
                env.push(op)
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let interactive = atty::is(Stream::Stdin);
    let verbose = interactive || atty::is(atty::Stream::Stdout);

    let mut env = Env::new();

    if interactive {
        push_from_args(&mut env)?;
        eval_and_print_till_terminal(&mut env)?;
        loop {
            eval_stdin(&mut env, verbose)?;
        }
    } else {
        /*
        could possibly achieve the desired effect by seeking back through the stack
        and evaluating the first command we come to?

        that would mean we could have

        rpn "3 +" << 4
        which would then rearnage to "3 4 +"

        */
        push_from_args(&mut env)?;
        push_from_stdin(&mut env)?;
        eval_and_print_till_terminal(&mut env)?;
        println!("");
    }

    Ok(())
}
