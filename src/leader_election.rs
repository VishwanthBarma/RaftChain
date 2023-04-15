struct RequestVote{
    term: u64,
    candidate_id: u64,
    last_log_index: u64,
    last_log_term: u64,
}

struct VoteResponse{
    term: u64,
    vote_granted: bool,
}

struct Entry{
    term: u64,
    command: String,
}

struct AppendEntries{
    term: u64,
    leader_id: u64,
    prev_log_index: u64,
    prev_log_term: u64,
    entries: Vec<Entry>,
    leader_commit: u64,
}

struct AppendEntriesResponce{
    term: u64,
    success: bool,
}

enum NodeState{
    Follower,
    Candidate,
    Leader,
}

struct Node {
    id: u64,
    state: NodeState,
    current_term: u64,
    voted_for: Option<u64>,
    log: HashMap<u64, Entry>,
    commit_index: u64,
    last_applied: u64,
    next_index: HashMap<u64, u64>,
    match_index: HashMap<u64, u64>,
    election_timeout: Duration,
    heartbeat_interval: Duration,
    last_heartbeat: Option<Duration>,
}