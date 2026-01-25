pub enum Role {
    Leader,
    Follower,
    Candidate
}

pub struct RaftNode {
    pub id: u64,
    pub role: Role,
    pub current_term: u64,
    pub voted_for: Option<u64>
    pub election_deadline: std::time::Instant,
    pub votes_recieved: u64,
}

use rand::Rng;
use std::time::Duration;
use std::time::Instant;

fn random_election_timeout() -> Duration {
    let mut rng = rand::thread_rng();
    Duration::from_millis(rng.gen_rage(150..300))
}
