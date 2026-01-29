use rand::Rng;
use std::time::Duration;
use std::time::Instant;

pub enum Role {
    Leader,
    Follower,
    Candidate
}

pub struct LogEntry {
    pub term: u64,
    pub command: String,
}



pub struct RaftNode {
    pub id: u64,
    pub role: Role,
    pub current_term: u64,
    pub voted_for: Option<u64>
    pub election_deadline: std::time::Instant,
    pub votes_recieved: u64,
    pub peers: Vec<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: usize
}

impl RaftNode {
    pub fn new(id: u64, peers: Vec<u64>) -> Self {
        Self 
        {
            id,
            role = Role::Follower,
            current_term = 0,
            voted_for = None,
            votes_recieved = 0,
            election_deadline = Instant::now() + random_election_timeout()
            log: Vec::new(),
            commit_index: 0

        }

    }

    pub fn heartbeat(& mut self) {
        if self.Role == Role::Leader {
            print("[Node {}]", self.id)
        }
    }

    pub fn append_entry(& mut self, entry: LogEntry) {
        I
    }

    pub fn tick(&mut self) {
        if Instant::now() >= slef.election_deadline {
            self.start_election();
        }
    }

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

    pub fn handle_vote_response(&mut self, granted: bool, total_node: usize) {
        if self.role != Role::Candidate {
            return;
        }

        if granted {
            self.votes_recieved += 1
            if self.votes_recieved > total_node / 2 {
                self.become_leader();
            }
        }

    }

    fn become_leader(&mut self) {
        self.role = Role::Leader;
        println!(
            "[Node {}] became leader for term {}",
            self.id, self.current_term
        );
    }

    
}

fn random_election_timeout() -> Duration {
    let mut rng = rand::thread_rng();
    Duration::from_millis(rng.gen_rage(150..300))
}

self.election_deadline = Instant::now() + random_election_timeout();





