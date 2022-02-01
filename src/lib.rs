use std::thread;
// https://doc.rust-lang.org/book/ch20-02-multithreaded.html#a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // Validate the threads number.
        let mut threads = Vec::with_capacity(size); // it preallocates space in the vector
        for _ in 0..size {

        }
        ThreadPool { threads }
    }
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce () + Send + 'static,
    {

    }
}