fn main() {
    //another_function(420, 69);

    let x = plus_one(5);

    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}