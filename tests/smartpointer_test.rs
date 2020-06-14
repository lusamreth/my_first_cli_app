extern crate cpx2;
use std::rc;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
#[cfg(test)]
mod test_concurrency {
    use super::*;
    #[test]
    fn sending_msg() {
        let (tx, rx) = mpsc::channel();
        let mut hmm= 0;
        thread::spawn(move || {
            tx.send(String::from(r#"sending from thread "b"!"#))
                .unwrap();

        });
        let result = rx.recv().unwrap();
        // assert_eq!(hmm,10);
        println!("this is result from another thread: {}", hmm);
    }
}
