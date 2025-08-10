# Traits

> Similar to abstract classes in Java and Interfaces in JavaScript but with some differences.

A `trait` defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use `trail bounds` to specify that a generic type can be any type that has a certain behaviour.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("User: {}", self.name)
    }
}
```

We can also have default implementations for methods in traits:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Default summary")
    }
}
impl Summary for User {
    // No need to implement summarize here, it will use the default implementation
}
impl Summary for User2 {
    fn summarize(&self) -> String {
        format!("User2: {}", self.name) // This will override the default implementation
    }
}
```

## Traits as parameters
What if you wanna create a function `notify` such that only structs that implement the `Summary` trait can be passed to it? You can do this by specifying the trait as a parameter type:
```rust
pub fn notify(item: &impl Summary) {
    println!("Notifying: {}", item.summarize());
}
```

## Trait bounds

The `impl Trait` syntax works for straightforward cases but is actually a syntactical sugar for a longer form known as `trait bounds`. It looks like this:

```rust
pub fn notify<T: Summary>(item: T) { //The  T generic should implement the Summary trait and then item is of type T
    println!("Notifying: {}", item.summarize());
}
```

Which means that `notify` can accept any type `T` as long as `T` implements the `Summary` trait. This is useful for more complex scenarios where you might want to specify multiple traits or use trait bounds in conjunction with other generics.

We can also bound a generic to multiple traits:

```rust
pub fn notify<T: Summary + AnotherTrait>(item: T) {
    println!("Notifying: {}", item.summarize());
}
```


# Lifetimes

Lifetimes are hard to digest.
Takes a lot of time to understand.

Lot of times the compiler will help you and guide you in the right direction.


Let us consider this example:

```rust
fn main() {
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest(str1, str2);
    }
    println!("The longest string is: {}", ans);
}


fn longest(a:String,b:String) -> String{
    if a.len() > b.len() {
        a
    }
    else{
        b
    }
}
```

It works perfectly. But let us change the function signature a bit where the longest fn instead of taking ownership of the strings, takes in two string slices:

```rust
fn main() {
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        let ans = longest(&str1, &str2);
    }
}


fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```
We shall see squiggliness in line ```fn longest(a: &str, b: &str) -> &str``` that Missing Lifetime Specifier.
Why ? We simply used string slices in-place of String.

The function longest takes str1 and str2 by borrowing them, the values are still owned by the original str1 and str2 variables. Their ownership isn't transferred even after returning a reference to one of them. If we set answer to be pointing to one of the two it will be a dangling reference. As str2 goes out of scope and according to Rust's ownership rules, the value of str2 will be dropped, leaving answer pointing to invalid memory.

Even though answer pointed to str1 which is in scope, it is still throw the same error. The Rust compiler does not know what the Lifetime of str1/str2 is in relation to the function longest.

It returns a borrowed value but the compiler could not be sure if it's coming from str1 or str2.

Therfore, the Rust Compiler needs us to tell how long the references are valid for or for how many lines (lifetime).
The compiler asks us to specify a relationship between the Lifetimes of both str1 and str2 which can be intersection based on their scopes.

## How to fix the error ? - Specify Lifetimes (generic lifetime annotations)


```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        let ans = longest(&str1, &str2);//Error- str2 does not live long enough
        println!("The longest string is: {}", ans);//works - as return value in ans is intersection of both scopes
    }
    // println!("The longest string is: {}", ans);//outside the scope
}
```
'a - Lifetime Generic Annotation. The return type is intersection of lifetimes of str1 and str2. Now, we will see error that `str2` does not live long enough if we use same code as before in main fn if we try printing ans outside the str2 scope, but it will not be an error if we print ans inside the scope where str2 is valid.