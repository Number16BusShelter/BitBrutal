use num_cpus;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel;
use std::thread::ThreadId;
use crate::core::combinations::CombinationGenerator;
use crate::core::components::HashComponents;

pub type CheckFn = fn(&str, &HashComponents) -> bool;

pub fn process_combinations(
    alphabet: String, 
    hash: String, 
    max_length: usize, 
    check_fn: CheckFn, 
    generator_state_file: &str,
    success_file: &str,
) {
    const BATCH_SIZE: usize = 1000; // Adjust as needed

    let (sender, receiver) = channel::bounded(BATCH_SIZE); 
    let receiver = Arc::new(Mutex::new(receiver));

    let generator = Arc::new(Mutex::new(CombinationGenerator::new(&alphabet, max_length, generator_state_file)));

    let producer = thread::spawn({
        let generator = Arc::clone(&generator);
        let sender = sender.clone();
        move || {
            loop {
                let mut generator = generator.lock().unwrap();
                if let Some(comb) = generator.next() {
                    // Save generator state
                    generator.save_state();
                    sender.send(comb.clone()).unwrap();
                } else {
                    break;
                }
            }
        }
    });

    let hash_components = HashComponents::new(&hash);
    let num_cpus = num_cpus::get() / 2; // get the number of logical cores
    let mut consumers = Vec::new();
    println!("Number of CPUs - {}", num_cpus);
    for _ in 0..num_cpus { 
        let receiver = Arc::clone(&receiver);
        let hash_components = hash_components.clone(); 
        let handle = thread::spawn(move || {
            let thread_id: ThreadId = std::thread::current().id();
            loop {
                // introduce a new scope for the channel read operation
                let received = {
                    let receiver = receiver.lock().unwrap();
                    if let Ok(received) = receiver.recv() {
                        Some(received)
                    } else {
                        None
                    }
                };
                if let Some(received) = received {
                    let check_result = check_fn(&received, &hash_components);
                    println!("Thread id: {:?}, Guess: {}, Result: {}", thread_id.as_u64(), received, check_result);
                    // If check_result is true, append the successful guess to success file
                } else {
                    // break the loop if there's nothing left in the channel
                    break;
                }
            }
        });
        consumers.push(handle);
    }

    producer.join().unwrap();

    for consumer in consumers {
        consumer.join().unwrap();
    }
}