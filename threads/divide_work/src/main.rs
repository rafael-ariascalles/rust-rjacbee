
fn main() {
    const N_THREADS: usize = 1;
    let operations: Vec<u32>  = (0..15000).collect();
    let mut thread_handles = Vec::new();

    let chunks = operations.chunks(N_THREADS);

    for chunk in chunks {
        let inner_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || {
            inner_chunk.iter().sum::<u32>()
        }))
    }

    let mut result_value = 0;
    for handle in thread_handles {
        result_value += handle.join().unwrap();
    }
    println!("Sum: {}", result_value);
}
