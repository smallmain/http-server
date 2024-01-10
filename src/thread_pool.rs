use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

/// 任务
pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// 线程池
pub struct ThreadPool {
    /// 线程集合
    threads: Vec<Option<thread::JoinHandle<()>>>,
    /// 任务发送器
    sender: Option<Sender<Job>>,
    /// 任务接收器
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

impl ThreadPool {
    /// 创建线程池实例
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let mut pool = ThreadPool {
            threads: Vec::with_capacity(size),
            sender: Some(sender),
            receiver: Arc::new(Mutex::new(receiver)),
        };

        for _ in 1..=pool.threads.capacity() {
            let receiver = pool.receiver.clone();
            pool.threads.push(Some(thread::spawn(move || {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => job(),
                    Err(_) => println!("shutdown."),
                }
            })));
        }

        pool
    }

    /// 将任务添加到线程池
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for thread in &mut self.threads {
            thread.take().unwrap().join().unwrap();
        }
    }
}
