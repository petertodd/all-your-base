use std::process::Command;

fn main() {
    println!("Securing the base..");
    Command::new("chmod 000 base")
            .output()
            .expect("Failed to execute process");
    println!("...base secured!");
}
