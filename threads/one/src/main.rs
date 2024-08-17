fn run_thread(n: i32) {
    println!("text from the spawned Thread number {}", n);
}

fn main() {
    println!("text from the main Thread");

    let mut thread_handles = Vec::new();
    for i in 0..5 {
        let thread_handle = std::thread::spawn(move || run_thread(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| {
        handle.join().expect("Thread panicked");
    });
}
