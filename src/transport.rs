use crate::rpc::Rpc;

pub struct Transport {
    pub queues: Vec<Vect<Rpc>>,
}

impl Transport {
    pub fn new(n: usize) -> Self {
        Self {
            queues: vec![Vec::new(); n],
        }
    }

    pub fn send(&mut self, target: usize, rpc: Rpc) {
        self.queues[target].push(rpc);
    }

    pub fn recv_all(&mut self, id: usize) -> Vec<Rpc> {
        let mut msgs = Vec::new();
        std::mem::swap(&mut msgs, &mut self.queues[id]);
        msgs
    }
}