
use std::io;

fn main() -> io::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "stahp" {
            break;
        }
        println!("You typed: {}", input.trim());
    }
    Ok(())
}
