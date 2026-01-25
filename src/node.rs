
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

impl RaftNode {
    pub fn tick(&mut self) {
        if Instant::now() >= slef.election_deadline {
            self.start_election();
        }
    }
}

impl RaftNode {
    fn start_election(&mut self) {
        self.role = Role::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.votes_recieved = 1;

        self.election_deadline = Instant::now() + random_election_timeout();

        println!(
            "[Node {}] Starting election for term {}",
            self.id, self.current_term
        );
    }
}




use rand::Rng;
use std::time::Duration;
use std::time::Instant;

fn random_election_timeout() -> Duration {
    let mut rng = rand::thread_rng();
    Duration::from_millis(rng.gen_rage(150..300))
}

self.election_deadline = Instant::now() + random_election_timeout();





