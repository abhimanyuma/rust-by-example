fn main() {
    // The {} signifies "holes" that will be filled in by variables that are
    // provided to the string
    println!("{} days", 31);

    // {} also allows you to specify numbers inside so that we can use
    // as positional arguments, {0} is the first argument, {1} the second
    // and so on
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // {} also takes in named arguments rather than positional arguments
    // this can be even more explicit
    println!("{name}: {designation}", name="Tony Stark", designation="Iron Man");

    // Numbers can be converted to various bases with :D where D is the base
    // signifier. It can be b - binary, o - octal, x - hex lower case, or
    // X - hex upper case.
    let number: u32 = 4207849484;
    println!("Number in decimal is {}", number);
    println!("Number in binary  is {:b}", number);
    println!("Number in octal   is {:o}", number);
    println!("Number in hex     is {:x}", number);
    println!("Number in hex     is {:X}", number);
}
