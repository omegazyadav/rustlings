// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// I AM DONE

fn main() {
    let number = "3"; // don't change this line
    println!("Number {}", number);
    
    // We need to implement the shadowing concept here. Shadowing allows you 
    // to define the variable with existing name along with different data type
    {
        
    let number: i32 = 3;

    }
    println!("Number {}", number);
}
