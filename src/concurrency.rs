use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

pub fn concurrent_execution() {
    let mut join_handles = Vec::new();
    let fib = vec![1, 1, 2, 3, 5, 8];
    let (sender, receiver): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    let fib = Arc::new(Mutex::new(fib));
    for i in 0..5 {
        let fib = fib.clone();
        let sender = sender.clone();
        let handle = thread::spawn(move || {
            {
                let mut fib = fib.lock().unwrap();
                fib.push(i);
                sender.send(i).unwrap();
            }

            println!("Hello from thread {}: {:?}", i, fib);
        });
        join_handles.push(handle);
    }

    drop(sender);

    for elem in receiver {
        println!("Received: {}", elem);
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
}
