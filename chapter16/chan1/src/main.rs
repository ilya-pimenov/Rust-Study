use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    // Take 2

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    // Take 3

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let send_logic = |tx: mpsc::Sender<String>, vals: Vec<String>| {
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    };

    thread::spawn(move || {
        send_logic(
            tx1,
            vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ],
        );
    });
    thread::spawn(move || {
        send_logic(
            tx,
            vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ],
        );
    });

    for received in rx {
        println!("Got: {received}");
    }

    /*

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
    */
}
