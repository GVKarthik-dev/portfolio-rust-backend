pub fn sample_boolean(n: u32){
    println!("Binary {:b}", n);
    println!("Hexadecimal {:X}", n);
    println!("Hexadecimal {:x}", n);
}

pub fn varibles(){
    let mut x: i16 = 5;
    println!("The value of x is: {}", x);
    x = 10;
    println!("The value of x is now: {}", x);
    const PI: f32 = 3.14;
    println!("The value of PI is now: {}", PI);

    let mut b:i16 = 16;
    println!("The value of b is: {}", b);
    {
        let b = 20;
        println!("The value of b is now: {}", b);
    }
    b = b+100;
    println!("The value of b is now: {}", b);
}

pub fn string_formating(name:String, place:String, friend:String, age:u8){
    println!("My name is {0}. \nOne day {0} went out for {1}, and met a {2} while {1}. \n{0} asked him what's your name he replayed {3} and {4} year's old", name, place, "guy", friend, age);
    /*
    This is how we coment multiple lines at once
    println!("This is a \nmulti-line \ncomment");
    
    */
}

pub fn mathematical(){
    let a: i32 = 5;
    let b: i32 = 10;
    // let c: f32 = 10.0/3.0
    // let d: f32 = 10.0
    println!("Addition: {}", a+b);
    println!("Subtraction: {}", a-b);
    println!("Multiplication: {}", a*b);
    println!("Division a/b: {}", a/b);
    println!("Division b/a: {}", b/a);
}

fn fibonacci(n: u32) -> Vec<u32> {
    let mut series = Vec::new(); // Create an owned vector to store the series

    if n == 0 {
        return series; // Return an empty vector if n is 0
    }

    series.push(0); // Initialize with the first value

    if n > 1 {
        series.push(1); // Initialize with the second value if n > 1
    }

    for i in 2..n {
        let next = series[i - 1] + series[i - 2]; // Calculate the next value
        series.push(next); // Add the next value to the vector
    }

    series // Return the ownership of the vector
}

fn main() {
    let n = 10;
    let fib_series = fibonacci(n); // Take ownership of the returned vector

    println!("Fibonacci series up to {} terms: {:?}", n, fib_series);
}

fn main(){
    println!("Testing");
}