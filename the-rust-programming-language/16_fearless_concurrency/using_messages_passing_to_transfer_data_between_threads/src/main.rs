use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // transmitter and receiver
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hi")).unwrap();
        let vals = vec![
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
}
