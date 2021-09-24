// mod th
// use std::process;
use std::any;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{process, thread};
// use std::any::Any;

fn main() {
    let v = vec![1, 2, 3];

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    let handle: JoinHandle<()> = thread::spawn(move || {
        let val = String::from("hello world");

        tx.send(val).unwrap();

        // println!("val: {}", val);

        println!("v: {:?}", v);

        let vals = vec![
            String::from("the"),
            String::from("child thread"),
            String::from("says"),
            String::from("hello"),
        ];

        for val in vals {
            tx.send(val).unwrap();

            thread::sleep(Duration::from_millis(1));
        }

        for i in 1..10 {
            println!("new thread: {}", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("and"),
            String::from("sends"),
            String::from("more"),
            String::from("messages"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let received = rx.recv().unwrap();

    println!("received: {}", received);

    for received in rx {
        println!("received vec: {}", received);
    }

    handle.join().unwrap_or_else(|v: Box<dyn any::Any + Send>| {
        process::exit(1);
    });
}
