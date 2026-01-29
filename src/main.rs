mod node;

use node::{RaftNode, Role};
use std::{thread, time::Duration};

fn main() {
    println!("Welcome to the distributed systems raft simulator")
    let node_ids = vec![1,2,3,4,5];
    let mut nodes: Vec<RaftNode> = node_ids
        .iter()
        .map(|id| {
            let peers = node_ids.iter().cloned().filter(|x| x != id).collect();
            RaftNode::new(*id, peers)
        })
        .collect();
}

loop {
    for i in 0..node.len() {
        nodes[i].tick();
        if nodes[i].role == Role::Candidate {
            for _ in &nodes[i].peers {
                nodes[i].handle_vote_response(true, nodes.len())
            }
        }
    }
    std::thread::sleep(Duration::from_millis(10))
}

