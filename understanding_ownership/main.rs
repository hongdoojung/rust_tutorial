fn main() {
    let mut s1 = String::from("hello");

    let len1 = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len1);

    change(&mut s1);

    let len1 = calculate_length(&s1);

    let s2 = first_word(&s1);

    let len2 = calculate_length(&s2);

    println!("The length of '{}' is {}.", s1, len1);
    println!("The length of '{}' is {}.", s2, len2);

    let s1 = no_dangle();

    let len1 = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len1);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}