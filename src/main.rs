use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Todo less arguments passed...");
    }
    let action = args[1].clone();
    let task = args[2].clone();
}

struct Todo {
    map: HashMap<String, String>
}
