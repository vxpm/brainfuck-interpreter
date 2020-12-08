use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Error: No brainfuck file specified.");
        std::process::exit(1);
    }

    match brainfuck_interpreter::run(&args[1][..]) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
