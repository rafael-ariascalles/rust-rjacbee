use std::thread;

fn thread_over() {
    println!("Threaded function {}", thread::current().name().unwrap());
}

fn main() {
    let handler = thread::Builder::new()
        .name("threaded_function".to_string())
        .stack_size(std::mem::size_of::<usize>() * 5)
        .spawn(thread_over)
        .expect("Failed to create a new thread");

    handler.join().unwrap();
}

