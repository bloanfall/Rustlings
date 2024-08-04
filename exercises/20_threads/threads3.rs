use std::{sync::{mpsc, Arc}, thread, time::Duration};

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<mpsc::Sender<u32>>) {
    let first_half_tx = Arc::clone(&tx);
    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending from first half: {val}");
            first_half_tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let second_half_tx = Arc::clone(&tx);
    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending from second half: {val}");
            second_half_tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();
        let queue_length = queue.length;

        let arc_tx = Arc::new(tx);
        send_tx(queue, arc_tx);

        let mut total_received: u32 = 0;
        for received in rx {
            println!("Got: {received}");
            total_received += 1;
        }

        println!("Number of received values: {total_received}");
        assert_eq!(total_received, queue_length);
    }
}
