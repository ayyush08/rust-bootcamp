fn main() {
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest(&str1, &str2);
        println!("The longest string is: {}", ans);
    }
    // println!("The longest string is: {}", ans); Error: `str2` does not live long enough
}


fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str { //ERROR: Missing Lifetime Specifier -this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}