use std::env;
use std::io::{self, Write};

fn main() {
  let args: Vec<_> = env::args().collect();

  if args.len() == 1 {
    // start up the REPL environment
    println!("Jihadi-C interpreter version 0.01");
    repl();
  } else if args.len() >= 1 {
    println!("running file {}. \n", args[1])
  } else {
    println!("Wrong usage");
  }
  return;
}


fn repl() {
  
  let mut buffer = String::new();

  loop {

    print!(">> ");
    let _ = io::stdout().flush();

    _ = io::stdin().read_line(&mut buffer);
  }
}
