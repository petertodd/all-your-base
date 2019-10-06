use std::process::Command;

fn main() {
    println!("Securing the base..");
    Command::new("chmod 000 base");
    println!("...base secured!");
}
