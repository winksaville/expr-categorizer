use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
enum Categories {
    Other = 0,
    Cat1,
    Cat2,
}

fn num_to_category(n: u8) -> Categories {
    match FromPrimitive::from_u8(n) {
        Some(Categories::Other) => Categories::Other,
        Some(Categories::Cat1) => Categories::Cat1,
        Some(Categories::Cat2) => Categories::Cat2,
        None => Categories::Other,
    }
}

fn category_to_num(c: &Categories) -> u8 {
    if let Some(n) = ToPrimitive::to_u8(c) {
        n
    } else {
        panic!("SNH because Categories has only values 0, 1 and 2.");
    }
}

fn categorize<F, T, C>(t: T, func: F) -> C
where
    F: Fn(T) -> C,
{
    func(t)
}

fn categorize_strings(s: &str) -> Categories {
    match s {
        "hello" => Categories::Cat1,
        "bye" => Categories::Cat2,
        _ => Categories::Other,
    }
}

fn main() {
    let c1 = Categories::Cat1;
    let c2 = Categories::Cat2;

    println!("c1: {c1:?}, c2: {c2:?}");

    println!("0: {:?}", num_to_category(0));
    assert_eq!(num_to_category(0), Categories::Other);
    println!("c1: {:?}", category_to_num(&c1));
    assert_eq!(category_to_num(&c1), 1);


    let c = categorize(1, num_to_category);
    println!(r#"categorize(1, num_to_category): {c:?}"#);
    assert_eq!(c, Categories::Cat1);

    let c = categorize("hello", |t| match t {
        "hello" => Categories::Cat1,
        "bye" => Categories::Cat2,
        _ => Categories::Other,
    });
    assert_eq!(c, Categories::Cat1);
    println!(r#"categorize("hello", closure): {c:?}"#);

    let c = categorize("bye", categorize_strings);
    println!(r#"categorize("bye", categorizing): {c:?}"#);
    assert_eq!(c, Categories::Cat2);

    let c = categorize_strings("something");
    println!(r#"categorize_strings"something"): {c:?}"#);
    assert_eq!(c, Categories::Other);
}

