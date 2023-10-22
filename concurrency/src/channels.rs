/*
    Message channels allow the developer to send data between threads.
    There can be multiple senders, but only one receiver.
    By doing so, we can ensure the ownership and borrowing system is respected.
    Ownership is transferred from the sender to the channel, and then to the receiver.
*/

use std::{sync::mpsc, thread, time::Duration};

pub fn run() {
    println!("=== Channels simple ===");
    channels_simple();

    println!("=== Channels multiple ===");
    channels_multiple();

    println!("=== Channels concurrent ===");
    channels_concurrent();

    println!("=== Channels game ===");
    channels_game();
}

fn channels_simple() {
    let (tx, rx) = mpsc::channel::<String>();

    let sender_thread_handle = thread::spawn(move || {
        println!("Second thread: Creating String on heap");
        let val = String::from("hi");

        println!("Second thread: Sending String to channel.");
        tx.send(val).unwrap();
        /*
           By calling send, we are transferring ownership of val to the channel, as we are not passing a reference.
           Note that we are unwrapping the result of send without handling possible errors, which means our program will panic if the channel is disconnected.
           In a real program, you should handle errors properly.
        */
    });

    println!("Main thread: Waiting to receive String from channel.");
    let received = rx.recv().unwrap(); // The String will be returned by recv() so that ownership is transferred to the received variable.
    println!("Got: {}", received);
    /*
       Note that the call to recv() is blocking, which means the main thread will wait until it receives a value from the channel.
       Again, we are unwrapping the result without handling possible errors.

       In a scenario where you don't want the thread to block, you can use try_recv() instead.
    */

    sender_thread_handle.join().unwrap();
}

fn channels_multiple() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        println!("Second thread: Creating Vec<String> on heap");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        println!("Second thread: Iterating over Vec<String> and sending Strings to channel every second.");
        for val in vals {
            tx.send(val).unwrap(); // Again, in a real program, you should handle errors properly.
            thread::sleep(Duration::from_secs(1));
        }

        println!("Second thread: Closing channel and dropping tx.");
        // tx is dropped here, which means the channel is disconnected.
    });

    println!("Main thread: Iterating over receiver");
    for received in rx {
        println!("Got: {}", received);
    }

    println!("Main thread: Seems like tx was dropped, so we are done!");
    // Note that, logically, we don't need to join the second thread, as this code is only reached when the second thread is done and the tx is dropped.
}

fn channels_concurrent() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // Again, in a real program, you should handle errors properly.
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("<this>"),
            String::from("<is>"),
            String::from("<the>"),
            String::from("<third>"),
            String::from("<thread>"),
        ];

        for val in vals {
            tx2.send(val).unwrap(); // Again, in a real program, you should handle errors properly.
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

enum GameProtocol {
    MoveUp {
        player: u8,
        amount: u8,
    },
    MoveDown {
        player: u8,
        amount: u8,
    },
    MoveLeft {
        player: u8,
        amount: u8,
    },
    MoveRight {
        player: u8,
        amount: u8,
    },
    Damage {
        player: u8,
        other_player: u8,
        amount: u32,
    },
}

fn channels_game() {
    let (tx, rx) = mpsc::channel::<GameProtocol>();

    let server_thread = thread::spawn(move || {
        for received in rx {
            match received {
                GameProtocol::MoveUp { player, amount } => {
                    println!("Player {} moved up by {}.", player, amount);
                }
                GameProtocol::MoveDown { player, amount } => {
                    println!("Player {} moved down by {}.", player, amount);
                }
                GameProtocol::MoveLeft { player, amount } => {
                    println!("Player {} moved left by {}.", player, amount);
                }
                GameProtocol::MoveRight { player, amount } => {
                    println!("Player {} moved right by {}.", player, amount);
                }
                GameProtocol::Damage {
                    player,
                    other_player,
                    amount,
                } => {
                    println!(
                        "Player {} damaged player {} by {}.",
                        player, other_player, amount
                    );
                }
            }
        }

        println!("Server thread: No clients connected anymore, server shutting down.");
    });

    tx.send(GameProtocol::MoveUp {
        player: 1,
        amount: 1,
    })
    .unwrap();

    tx.send(GameProtocol::MoveDown {
        player: 1,
        amount: 1,
    })
    .unwrap();

    tx.send(GameProtocol::MoveLeft {
        player: 1,
        amount: 1,
    })
    .unwrap();

    tx.send(GameProtocol::MoveRight {
        player: 1,
        amount: 1,
    })
    .unwrap();

    tx.send(GameProtocol::Damage {
        player: 1,
        other_player: 2,
        amount: 10,
    })
    .unwrap();

    drop(tx); // This will disconnect the channel, which will cause the server thread to stop iterating over the channel and stop.

    server_thread.join().unwrap();
}
