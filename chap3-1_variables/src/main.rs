fn main() {
    // Variables are immutable
    // Cannot assign twice to immutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // You can declare a new variable with the same name as a previous variable
    let number = 5;
    let number = number + 1;
    {
        let number = number * 2;
        println!("The value of 'number' in the inner scope is: {number}");
    }
    println!("The value of 'number' is: {number}");
}
