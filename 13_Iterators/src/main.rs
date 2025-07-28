use std::vec;

fn main() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // let mut iter = v.iter_mut();

    // for val in iter{
    //     *val *= 2; 
    // }

    // while let Some(val) = iter.next() {
    //     print!("{} ", val); // Output: 12345
    // }
    //whenever .next() is called on an iterator, it starts to consume the values from the iterator and moves the iterator forward. Therefore, it return Option because it may not always have a value to return(the end of the iterator).
    // println!("{:?}", v); // Output: [2, 4, 6, 8, 10]

    // let nums = vec![1, 2, 3, 4, 5];

    // for num in nums{ //uses the IntoIterator trait to convert the vector into an iterator
    //     println!("{}", num); // Output: 12345
    // }

    // println!("{:?}", nums); //Error: cannot borrow `nums` as immutable because it is also borrowed as mutable

    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    let sum:i32 = v1_iter.sum();

    println!("Sum: {}", sum); // Output: Sum: 15)

    // for i in v1_iter { //ERR: Use of moved value: `v1_iter`
    //     println!("{}", i); // This will not print anything because the iterator has been consumed
    // }

    let v1_filter_iter = v1.iter().filter(|x| *x % 2 == 0);

    for x in v1_filter_iter {
        println!("Even Value: {}", x);
    }
}


//Assignment:
//Write the logic to first filter all odd values then double aech and create a new vector