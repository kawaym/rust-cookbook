use std::{collections::HashMap, io};

#[derive(Debug)]
struct Department {
    name: String,
}

#[derive(Debug)]
struct Employee {
    name: String,
    department: Department,
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    //panics
    //let does_not_exist: &i32 = &v[100];
    //println!("{}", does_not_exist);

    //does not panic
    let does_not_exist: Option<&i32> = v.get(100);
    match does_not_exist {
        None => println!("Does not exist."),
        _ => println!("Does exist."),
    }

    let mut add_v = vec![0, 50, 100, 150, 200];

    for i in &mut add_v {
        *i += 50;
    }

    println!("{:?}", add_v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();

    let data = "initial contents";

    let data_s = data.to_string();

    println!("{data}");

    let word = String::from("Mérde");

    for i in word.chars() {
        println!("{i}");
    }

    for i in word.bytes() {
        println!("{i}");
    }

    let mut scores: HashMap<String, i64> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:#?}");

    let mut team_name = String::from("Blue");
    let mut score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score of {team_name} is {score}");

    team_name = String::from("Green");
    score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score of {team_name} is {score}");

    println!("{scores:#?}");

    let values = vec![1, 2, 3, 3, 4, 5, 3, 2, 2, 2, 2, 2, 2];
    let (median, mode) = get_median_mode(values.clone());

    println!(
        "For the array {:#?} the median is {} and the mode is {}",
        values, median, mode
    );

    let pig_latin = convert_to_pig_latin("This phrase is in pig-latin".to_string());

    println!("{pig_latin}");

    add_to_workers();
}

fn get_median_mode(mut list: Vec<i32>) -> (f64, i32) {
    list.sort();
    let median: f64;
    let mut mode: i32 = 0;

    let middle_point = list.len() / 2 as usize;

    if list.len() % 2 == 1 {
        median = list[middle_point] as f64
    } else {
        median = (list[middle_point] + list[middle_point - 1]) as f64 / 2.0
    }

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for integer in list {
        let count = hash.entry(integer).or_insert(0);
        *count += 1;
    }

    for key in hash.keys() {
        if hash[key] > hash.get(&mode).copied().unwrap_or(0) {
            mode = *key
        }
    }

    (median, mode)
}

fn convert_to_pig_latin(phrase: String) -> String {
    let mut new_phrase = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for word in phrase.split_whitespace() {
        let first_letter = match word.chars().nth(0) {
            Some(letter) => letter,
            None => ' ',
        };

        let remaining_word = match word.get(1..) {
            Some(chars) => chars,
            None => " ",
        };

        if !word.starts_with(vowels) {
            new_phrase += remaining_word;
            new_phrase += "-";
            new_phrase += &first_letter.to_string();
            new_phrase += "ay";
            new_phrase += " "
        } else {
            new_phrase += word;
            new_phrase += "-hay";
            new_phrase += " "
        }
    }
    new_phrase
}

fn add_to_workers() {
    println!("Insert a employee with: Add [Employee] to [Department]:");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let split_phrase: Vec<&str> = buffer.split_whitespace().collect();

    // TODO: Implement with regular expressions

    let employee_name = match split_phrase.get(1) {
        Some(name) => name,
        None => " ",
    };

    let department_name = match split_phrase.get(3) {
        Some(name) => name,
        None => " ",
    };

    if employee_name == " " || department_name == " " {
        println!("Please write as the example")
    };

    let department = Department {
        name: department_name.to_string(),
    };

    let employee = Employee {
        name: employee_name.to_string(),
        department: department,
    };

    println!("{:#?}", employee);
}
