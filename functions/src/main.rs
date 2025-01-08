fn main() {
    println!("Hello, world!");

    another_function(5);

    let v = if 7 == 4 {
        7
    } else if false {
        9
    } else {
        27
    };

    println!("value: {v}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
