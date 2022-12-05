use rand::*;

fn demo01() {
    let a: i32 = 6;
    let b = 32;

    println!("b: {b}");

    if a <= 6 {
        println!("a <= 6: {a}");
    } else if a > 7 {
        println!("a > 7: {a}");
    }
}

fn demo02() {
    for number in (1..4).rev() {
        println!("demo02 number: {number}");
    }
}

fn demo03() {
    let r: u128 = random();
    println!("demo3: {r}");

    let s1 = String::from("link");
    let mut s2 = s1;

    s2.push_str("-drone");

    // println!("s1:{s1}");
    println!("s2:{s2}");

    let v1 = 1;
    let v2 = v1;

    println!("v1:{v1}");
    println!("v2:{v2}");
}

fn demo04() {
    struct User {
        age: u32,
        name: String,
    }

    let user = User {
        age: 18,
        name: String::from("link-drone"),
    };

    println!("user.age: {}", user.age);
    println!("user.name: {}", user.name);
}

fn demo05() {
    let usd1 = coin_to_usd(Coin::BTC);
    println!("usd1: {usd1}");

    let usd2 = coin_to_usd(Coin::ETH);
    println!("usd2: {usd2}");
}

enum Coin {
    BTC,
    ETH,
}
fn coin_to_usd(coin: Coin) -> u32 {
    return match coin {
        Coin::BTC => 17300,
        Coin::ETH => 1290,
    };
}

fn main() {
    demo01();

    demo02();

    demo03();

    demo04();

    demo05();
}
