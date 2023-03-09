use std::sync::{Arc, Mutex};

fn main() {
    let mut vec = Vec::new();
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    for x in 0..10 {
        let data_r = data.clone();
        vec.push(std::thread::spawn(move || {
            let mut data = data_r.lock().unwrap();
            data[x] += 1;
            println!("Thread No.{}", x)
        }))
    }

    for elem in vec {
        let _non = elem.join();
    }

    dbg!(data);
}
