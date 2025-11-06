fn main() {
    let mut x = 5;
    x -= 1;
    println!("The value of x is: {x}");
    // constants need to have a annotated data type and need to be set to constant value,
    // not a value that can only be compiled at run time.
    const X: i32 = 6;
    println!("The value of x is: {X}");

    let x = 5;

    let x = x + 1;

    { 
        // when a shadowed variable goes out of scope it goes back to its previous state.
        // shadowing is most often used for change of type without filling up the name space. 
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}