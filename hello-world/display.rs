// This imports the standard formatting library the `fmt` module.
use std::fmt;

// This is the struct on which we will be implementing fmt::Display
// Structure is a struct that contains an i32
struct Structure(i32);

// To use {} than just {:?} we need to implement the Display trait for the 
// struct Structure
impl fmt::Display for Structure {
    // A trait requires certain methods with defined signature, for this
    // this is the only method required, once which outputs an fmt::Result
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

fn main() {
    let st = Structure(32);
    
    println!("Structure is {}", st);

}