use std::fs;

fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    loop {
        print!("Prompt>: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        if s.eq_ignore_ascii_case("ls") {
           dir();
        }
        if s.eq_ignore_ascii_case("exit") {
            break;
        }
        else {
            //println!("You typed: {}",s);
            s = String::new();
        }
    }
}

fn dir() {
    let paths = fs::read_dir("./").unwrap();
    // TODO: Fix the println to print the appropriate directory info 
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
