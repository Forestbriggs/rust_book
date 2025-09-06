fn main() {
    let mut v = Vec::new();

    let _v_infered = vec![1,2,3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let new_v = vec![100, 32, 57];
    for i in &new_v {
        println!("{i}");
    }

}
