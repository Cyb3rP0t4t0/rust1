use std::io;
fn main() {
    let mut x = 20;
    println!("\nx is: {}", x); //It's a macro, I don't even now what macro means :'(
        
        { //This is a scope, cool shit. Everything you write will stay inside in this scope. And you can use variables from the outside like:
            let x = x - 2;
            println!("\nx is: {}", x);
        }

    x += 1; //Works with let also, I'll overwrite the first variable yee yee ass language
    println!("\nx is now: {}", x); //Just like in C#

    const SEX:u32 = 101; //Always use uppercase, in the rules and also you can't change the variable type:(
    println!("\n{}", SEX);

    let mut tup: (i32,bool,char) = (1, true, 's');

    tup.0 = 100;
    println!("\n{}", tup.0);

    let mut arr: [i32 ; 10] = [1,2,3,4,5,6,7,8,9,10];
    arr[5] = 69;
    println!("\n{}", arr[5]);

    println!("\nType something big boy.");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed line");
    println!("\n{}", input);
}
