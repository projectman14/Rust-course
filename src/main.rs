#![deny(clippy::all)]

fn main() {
    let mut first_name = "Foo"; // in rust we basically uses the snake_case naming convention
    println!("Hello , {}!", first_name);
    first_name = "Fun";
    println!("Hello , {}!", first_name);

    let age: i32 = 18;
    println!("Hello , {}!", age);
    let size = 35i32;
    println!("Hello , {}!", size);

    //concept of shadowing in rust it is not in many languages but rust has
    //it can also change the data type of the variable
    let distance: f64 = 5.5;
    println!("Hello , {}!", distance);
    let distance = "Hello";
    println!("Hello , {}!", distance);

    //variable shadowing using blocks
    //converting a string slice into string
    let _name: &str = "Foo";
    {
        let _name: String = _name.to_string();
    }
    println!("Hello , {}!", _name);

    //constants in rust must be upper case and type has to define the type of the variable
    //and must be given the initial value
    const MY_AGE: i8 = 25;
    println!("Hello , {}!", MY_AGE);

    //tuples in rust
    let personal_data = (21, "Foo");
    //to extract the data from the tuple
    let _age = personal_data.0;
    println!("Hello , {}!", _age);
}
