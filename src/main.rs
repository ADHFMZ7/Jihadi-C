use std::env;
use std::io::{self, Write};

// mod token;
// mod lexer;

fn main() {
  let args: Vec<_> = env::args().collect();

  if args.len() == 1 {
    // start up the REPL environment
    println!("Jihadi-C interpreter version 0.01");
    repl();
  } else if args.len() >= 1 {
    println!("running file {}. \n", args[1])
  } else {
    println!("Impropper usage.");
  }
  return;
}


fn repl() {
  
  let mut buffer = String::new();


  loop {

    print!(">> ");
    io::stdout().flush().unwrap();
 
    match io::stdin().read_line(&mut buffer) {
      Ok(0) => {
        break;
      }
      Ok(n) => {
        println!("length: {}", n);
        run(&buffer);
      } 
      Err(err) => {
        println!("error; {err}");
        break;
      }
    }
    buffer.clear();
  }
  println!("");
}

fn run(buffer: &str) {
  if buffer.len() >= 2 {
    print!("{}", buffer);
    io::stdout().flush().unwrap();
  }
}
