use std::thread;

fn thread_over() {
    println!("Threaded function {}", thread::current().name().unwrap());
}

fn main() {
    thread::Builder::new()
        .name("THREAD-1-0-0".to_string())
        .stack_size(std::mem::size_of::<usize>() * 5)
        .spawn(thread_over)
        .expect("Failed to create a new thread")
        .join()
        .expect("Failed to join the thread");
}

