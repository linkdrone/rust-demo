fn demo01() {
    let a: i32 = 6;

    if a <= 6 {
        println!("a <= 6: {a}");
    } else if a > 7 {
        println!("a > 7: {a}");
    }
}

fn demo02() {
    for number in (1..4).rev() {
        println!("demo02 number: {number}")
    }
}

fn main() {
    demo01();

    demo02();
}
