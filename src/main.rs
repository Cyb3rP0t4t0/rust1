use std::io;
fn main() {
    /*let mut x = 20;
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

    let z = 100_000i64; //What type of variable it is going to be. Also able to use _ like 10_i8 e.t.c / and as
    let u = i8::MAX as i32; //Gonna be the 8bit's max

    println!("{}", z / (u as i64)); //Converting from i32 to i64

    let mut input1 = String::new();

    io::stdin().read_line(&mut input1).expect("Oopsi");
    let int_input: i64 = input1.trim().parse().unwrap();

    println!("{}", int_input);
    
    let cond = 2 > 3;

    let cond2 = cond && true;

    println!("{}", cond2);*/

    let food = "cookie";

    if food == "cookie" {
        println!("\nIt is cookie!");
    } else {
        println!("\nIt is not cookie...");
    }

    test();
    calc(8,34);

    let num = { //statement can hold an expression
        let x = 3;
        x + 1 //Don't close it cuz we wanna return it!
    };

    println!("{}", num);
    
    let mut st1 = String::new();
    st1.push('A'); //Adds an A
    st1.push_str("word"); //Same but adds  it to the end 

    for word in st1.split_whitespace(){
        println!("{}", word);
    }  //No spaces

    let st2 = st1.replace("A", "Another");  //Gets a value and replaces one
    println!("{}", st2);
    
    
    let st3 = String::from("x r t z k k a m c d");
    let mut v1: Vec<char> = st3.chars().collect(); //A vector gets the string.
    v1.sort();  //It sorts them
    v1.dedup();  //Removes duplicates

    for char in v1 {
        println!("{}", char);
    }   //Prints out

    let st4 = "Random string";
    let mut st5 = st4.to_string();
    println!("{}", st5);

    let st6 = &st5[0..6];
    println!("String Lenght : {}", st6.len()); //Gives us the lenght of a string

    st5.clear();  //Deletes the string
    
    let st6 = String::from("Just a");
    let st7 = String::from("Black man shaking his booty.");
    let st8 = st6 + &st7;

    for char in st8.bytes(){
        println!("{}", char);
    }
}

fn test() {
    println!("\nTested!");
}

fn calc(x: i64, y: i64) {
    println!("\nSum: {}", x + y);
}

