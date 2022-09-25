use std::collections::HashMap;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone,Debug)]
pub struct SubgraphRunner<C, T>
{
    my_hashmap: HashMap<C, T>,
    num:i32,
}
impl<C, T> SubgraphRunner<C, T>{
    pub fn new()->Self {
        Self { my_hashmap: HashMap::new(), num: 0}
        
    }
}


fn run(arcSelf:Arc<Mutex<SubgraphRunner<i32,i32>>>) {

    let mut list = Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut handles = vec![];
    for i in 0..list.len(){
        let a = arcSelf.clone();
        
        let c = list.pop().unwrap();
        let handle = thread::spawn(move||{
            let mut b = a.lock().unwrap();
            b.my_hashmap.insert(c, c);
            b.num+=1;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let runner:Arc<Mutex<SubgraphRunner<i32,i32>>> = Arc::new(Mutex::new(SubgraphRunner::new()));
    run(runner.clone());
    println!("Hello, world!{:?}",runner.lock().unwrap());
}
