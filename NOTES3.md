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