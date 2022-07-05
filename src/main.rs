use std::{sync::{mpsc, Mutex}, thread, time::Duration};

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

//----- Simple thread spawning -----//

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

//----- Message passing concurrency -----//

// One approach to safe concurrency (popular in go) is message passing.
// Threads or actors communicate by sending messages to each other containing data.
// "Do not communicate by sharing memery, share memory by communicating".
// This is done in rust through channels with transmitters and receivers.
#[allow(dead_code)]
fn message_passing() {
    let (tx, rx) = mpsc::channel(); // multiple producer single consumer
                                    // there can only be one recieving end
                                    // tx = transmitter, tx = reciever

    // sending a value through the transmitter
    // transmitter must be owned by the spawned thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // -> Result<T, E> incase rx dropped
                               // if val is used here will panic
                               // because it's a borrow of a move
    });

    // recieve message while blocking main thread until value received
    let recieved = rx.recv().unwrap(); // receiviever takes ownership of val
    println!("Got: {}", recieved) // -> "hi"

    // try_recv will return a non blocking Result which can be polled periodically for
    // a value.
}

#[allow(dead_code)]
fn sending_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        // rx can be treated as an iterator
        println!("Got: {}", recieved)
    }
}

#[allow(dead_code)]
fn clone_transmitter() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx { // will recieve values form tx and tx1
        println!("Got: {}", recieved)
    }
}

// Shared state concurrency

// Message passing almost inherently implies ownership. Once a message has been sent
// it should no longer be possible to use that message in the sending part of the code. 
// Shared-state concurrency is like multiple ownership, with multiple thread accessing the 
// same memory location at the same time.

// Mutexes allow for only one thread to access some data at a given time. This is done through
// locks (guarding the data through mutexes).

// Managing mutexes is harder than channels, as locks need to be aquired and then released.


#[allow(dead_code)]
fn use_mutex(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // aquire lock on m so that it can be changed
                                         // this is enforced by the type system. 
        *num = 6;
    }

    println!("m = {:?}", m);
}




