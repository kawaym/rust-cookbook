fn main() {
    let mut s: String = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let r: &str = "hello";

    println!("{r}");

    let mut s1: String = String::from("hello");

    s1.push_str(", world!");

    let s2: String = s1;

    takes_ownership(s2);

    // println!("{s2}");

    let x: i32 = 5;

    makes_copy(x);

    println!("{x}");

    let s3 = gives_ownership();

    println!("{s3}");

    let s4: String = s3.clone(); // Deep copy of s1

    let s5: String = takes_and_gives_back(s4);

    println!("{s5}");

    let mut s6: String = String::from("hello");

    change(&mut s6);

    println!("{s6}");

    let mut s7: String = String::from("Test of a multi-worded phrase");

    let word = first_word(&s7);

    s7.clear();
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{some_string} string");
    some_string
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
