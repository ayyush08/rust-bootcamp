fn main() {
    let x = 99;
    let is_even = is_even(x);
    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }

    for i in 0..10 {
        //runs for 0 to 9
        print!("{} ", i);
    }

    let sentence = String::from("My Name is Curator");
    let first_word = get_first_word(sentence);
    println!("\nFirst word: {}", first_word);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn get_first_word(sentence: String) -> String {
    // let ans = String::new()
    //or
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
