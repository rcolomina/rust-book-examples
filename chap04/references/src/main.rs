fn main() {
    let s1 = String::from("hello");

    // the action of created a reference is called borrowing

    // unmutable borrowing reference
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    // Mutable borrowing reference
    let mut s2 = String::from("hello");    
    change(&mut s2);
    println!("s2 is {s2}");

    // Non-lexical lifetime NLL
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let no_dange = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

