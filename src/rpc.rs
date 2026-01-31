#[derive(Debug, Clone)]
//In distributed systems we must used RPCs as we do not know the state of the of a different dsystem and we may disrupt the process of that sustem.
//So instead we send a message describing the process to that other system.
//Each enum variant meants if this were a local system. I would call this method


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