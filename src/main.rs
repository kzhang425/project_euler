mod problems;
mod getter;
mod solutions;
// Import all problem "functions" to call directly here
use solutions::Solutions;


fn main() {
    let mut x = Solutions::new();
    println!("PROJECT EULER PROBLEMS");
    loop {
        
        x.do_problem();
        if ask_exit() == "q" {
            break;
        } else {
            continue;
        }
        
    }


}

fn ask_exit() -> String {
    println!("Would you like to quit? Type \"q\" without quotes to exit safely");
    let mut character = String::new();
    std::io::stdin().read_line(&mut character).unwrap();

    String::from(character.trim())

}