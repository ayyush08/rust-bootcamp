use std::{sync::mpsc, thread};

fn main() {

    // let (tx,rx) = mpsc::channel();


    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     println!("Sending: {}", val);
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv();
    // match received {
    //     Ok(v) => println!("Received: {}", v),
    //     Err(e) => println!("Error: {}", e),
    // }
    add_big();
}

// Assignment: Write a code that finds sum from 1 to 10^8. Use threads to make sure you use all cores on your machine. Remember its "multiple producers" and "single consumer" model.

//Example ans:

fn add_big() {
    let (tx, rx) = mpsc::channel();

    for i in 0..8{
        let producer = tx.clone(); // Clone the transmitter for each thread
        
        thread::spawn(move || {
            let mut sum: u64 = 0;
            for j in i*10000000..(i+1)*10000000-1{
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx); // Close the original transmitter to avoid hanging otherwise receiver will keep waiting thinking there's more to come

    let mut final_sum: u64 = 0;

    for val in rx {
        final_sum = final_sum + val;
        println!("Received: {}", val);
    }

    println!("Final Answer: {}", final_sum);
}