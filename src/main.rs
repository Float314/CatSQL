use colorize::*;
use rustyline;

mod dbactions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    println!("{}", "CatSQL v0.1.0 Development Alpha".blue().bold());

    let mut rl = rustyline::DefaultEditor::new()?;
    loop {
        let readline = rl.readline("catsql> ");
        match readline {
            Ok(line) => println!("smth0"),
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}", "Ctrl-C".red().bold());
                return Err("Keyboard Interrupt".into());
            }
            Err(e) => println!("{}", dbg!(e))
        }
    }

    Ok(())
}
