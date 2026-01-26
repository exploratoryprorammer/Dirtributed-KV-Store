mod node;

fn main() {
    println!("Welcome to the distributed systems raft simulator")
}

loop {
    node.tick();
    std::thread::sleep(Duration::from_millis(10))
}

