
//Write a function that takes a string as an input and returns the first word from it
// fn main() {
//     let name = String::from("Hello World");
//     let ans = first_word(name); //duplicates hello world into new string in heap memory
//     println!("The first word is: {}", ans);
// }

// fn first_word(str: String) -> String{
//     let mut ans = String::from("");
//     for c in str.chars() {
//         if c == ' ' {
//             break;
//         }
//         ans.push_str(&c.to_string());
//     }
//     return ans;
// }

//PROBLEMS WITH ABOVE METHOD
//1. It duplicates the string in heap memory, and takes double the memory
//2. If the 'name' string gets cleared, the 'ans' string still holds the value as 'hello';
//3 What we want is a 'view' of original string, not a copy of it. and that is why slices are used in Rust

fn main(){
    let  word = String::from("Hello World");
    let word2 = &word[0..5]; //slice of the string from index 0 to 5 (exclusive)
    //type of word2 is &str, which is a string slice
    // word.clear(); //ERR:Cannot borrow 'word' as mutable because it is also borrowed as immutable(without mut)
    //While you have an immutable reference to a value, you cannot mutate that value.
    println!("The first word is: {}", word2);
    
    //Note: The slice is a reference to the original string, so it does not take
    //up additional memory for the string itself, only for the slice metadata (start and end

    println!("First word using function: {}", first_word(&word)); //passing a reference to the string

    let v = [1,2,3];
    println!("Slice of array: {:?}", &v[0..2]); //slice of the array from index 0 to 2 (exclusive)
}


fn first_word(str: &String) -> &str{
    let mut space_index = 0;
    for i in str.chars()
    {
        if i == 'W' {
            break;
        }
        space_index += 1;
    }
    &str[0..space_index] //returning a slice of the string from index 0 to space_index (exclusive);
}