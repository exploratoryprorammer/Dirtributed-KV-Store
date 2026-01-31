#[derive(Debug, Clone)]

pub enum Rpc {
    RequestVote {
        term: u64
        candidate_id: usize,
    },
    RequestVoteResponse {
        term: u64,
        vote_granted: bool,
        from_id: usize,
    },
}