use std::{thread, time::Duration, sync::mpsc};

/*
 * Design concerns - Rust and concurrency
 *
 * Languages with a runtime can afford to abstract concurrency with models that 
 * take control away from the user. Lower level langauges like rust, without a large 
 * runtime are expected to have fewer abstractions over the hardware and to have 
 * soluitons for concurrency with optimal performance. Rust packages provide libraries 
 * with greater abstractions, but the standard library provides thread spawning, 
 * message-passing and shared-state concurrency. The threads are also implemented 1:1 
 * operating system threads.
 */

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn spawn_threadsa() {
    // spawning a thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // these should print concurrently
    for i in 1..5 {
        println!("number {} from main thread", i);
    }

    // there is no guarantee the spawned thread will be able to finish 
    // before the main one. Unless a handle is used.

    handle.join().unwrap(); // blocks current thread
}

#[allow(dead_code)]
fn closures_and_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    }); // cannot borrow v because there's no way of know how long 
        // thread will last for. Must specify type of capture as move


    
    handle.join().unwrap();
}

// Lifetimes help ensure concurrent safety at runtime by checking
// potential sources of error at runtime


// One approach to safe concurrency (popular in go) is message passing. 
// Threads or actors communicate by sending messages to each other containing data.
// "Do not communicate by sharing memery, share memory by communicating".
// This is done in rust through channels with transmitters and receivers.
#[allow(dead_code)]
fn message_passing(){      
    let (tx, rx) = mpsc::channel(); // multiple producer single consumer
                                    // there can only be one recieving end 
                                    // tx = transmitter, tx = reciever

    // sending a value through the transmitter
    // transmitter must be owned by the spawned thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // -> Result<T, E> incase rx dropped
    });

    // recieve message while blocking main thread until value received
    let recieved = rx.recv().unwrap(); 
    println!("Got: {}", recieved) // -> "hi"

    // try_recv will return a non blocking Result which can be polled periodically for
    // a value. 


}


