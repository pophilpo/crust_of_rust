use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}

pub struct Reciever<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Reciever<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

fn channel<T>() -> (Sender<T>, Reciever<T>) {
    let inner = Inner {
        queue: Mutex::default(),
    };

    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(),
        },
        Reciever {
            inner: inner.clone(),
        },
    )
}
