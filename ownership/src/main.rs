fn main() {
    let mut s = String::from("hello");

    takes_ownership(&mut s);

    println!("String in main: {s}");
}

fn takes_ownership(s: &mut String) {
    println!("String in takes_ownership: {s}");
    s.push_str(", boomers");
}


