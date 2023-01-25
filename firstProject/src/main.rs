#![allow(non_snake_case)]

fn main() {
    // Print statement
    println!("Hello, world!");

    // Declaring a variable
    let age = 10;
    let name = "Shuban";
    let weight = 75.9;
    let boy = true;

    // Use formatting and print the values of variables
    println!("My name is : {}" , name);
    println!("My age is : {}" , age);
    println!("My weight is : {}" , weight);
    println!("I am a boy : {}" , boy);

    // Declare immutable variable
    let y = 10;
    println!("The value of y is : {}" , y);

    // This will return errors
    // y = 15;
    // println!("The value of y is : {}" , y);

    // Sets a mutable variable
    let mut x = 10;
    println!("The value of x is : {}" , x);

    // Updates the value of mutable variable x
    x = 15;
    println!("The value of x is : {}" , x);

    // Demonstration of scopes
    let mut z = 1;
    println!("The value of z is : {}" , z);

    // Creates a scope and this scope can access variables outside the scope
    {
        let z = 10;
        println!("The value of z is : {}" , z);
    }

    // Uses the variable in the same scope and can't access the inner scope variable
    z = z + 1;
    println!("The value of z is : {}" , z);


}
