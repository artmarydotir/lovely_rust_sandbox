// var hold primitive data or refrence to data
// var are immutable by default
// rust is a block-scope language
// aaah we use let & const keyword like javascript to create var :)

pub fn run() {
    let name = "Mary";
    let mut age = 25;

    // reassign != cannot assign twice to immutable variable
    // but we can add mut keyword to mutate
    age = 40;
    println!("My name is {} and im {}", name, age);  
    
    // Define const always have to tell the type
    const ID: i32 = 001;
    println!("id is {}", ID);

    // multiple assign 
    let (a, b) = ("aa", "bb");
    println!("a and b are : {} {}", a, b);
}