fn run_thread(n: i32) {
    println!("text from the spawned Thread number {}", n);
}

fn do_math(i: i32) -> i32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    };
    n
}

fn main() {
    println!("text from the main Thread");

    let mut thread_handles = Vec::new();
    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| {
        println!("Sum Multiply: {}", handle.join().unwrap());
    });
}
