pub fn run() {
    // print to console
    println!("hi from a public function");

    // basic formatting
    println!("Number: {}", 1);
    println!("i am var {} and {}", 1,2);

    // Positional Argument
    println!("{0} is a {1} developer, {0} likes to learn Rust",
    "Mary", "front-end");

    // Named Argument 
    println!("{name} likes to {action} everyday",
        name = "Mary",
        action = "Code"
    );
    // placeholder traits 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug 
    println!("{:?}",(1,true,"hello") );

    // Basic Math 
    println!("10 + 5 : {}", 10+5);
}