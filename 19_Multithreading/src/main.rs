use std::thread;
use std::time::Duration;
fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {i} from the spawned thread");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    
    // for i in 1..5 {
    //     println!("Hi number {i} from the main thread");
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap(); //Wait for the spawned thread to finish and only then proceed

    let v = vec![1, 2, 3];
    // thread::spawn(|| { //ERR: May outlive borrowed value `v` so we gotta use move keyword to force ownership of the variable v (u cant use it after this though)
    //     println!("Here's a vector: {:?}", v);
    // });

    thread::spawn(move || { //Now v is owned by the spawned thread
        println!("Here's a vector: {:?}", v);
    });
}
