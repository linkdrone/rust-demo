#[test]
fn demo01() {
    let a = 6;
    let b = 32;

    println!("b: {b}");

    if a <= 6 {
        println!("a <= 6: {a}");
    } else if a > 7 {
        println!("a > 7: {a}");
    }
}

#[test]
fn demo02() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
