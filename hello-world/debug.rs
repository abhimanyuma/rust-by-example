// This "derives" the fmt::Debug implementation for `Structure`
#[derive(Debug)]
struct Structure(i32);

// We can even nest structures and fmt::Debug will still work.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Now this allows us to use the debug directive :? in printing
    println!("{:?} months in a year.", 12);

    // Strings by default will print normally but using debug it will
    // add the quotes since the debug representation shows that the
    // content is a string
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // fmt::Debug however has a standard mechanism that we can't control on how
    // the output will be printed.
    println!("Now {:?} will print!", Deep(Structure(7)));
}