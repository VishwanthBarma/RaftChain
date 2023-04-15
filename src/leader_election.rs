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

enum LogEntry{
    Heartbeat{ term: u64, peer_id: String },
}

struct Raft {
    id: u64,
    peers: Vec<u64>,
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

impl Raft {
    fn new(id: u64, peers: Vec<u64>) -> Self{
        let mut raft = Raft {
            id,
            peers,
            state: NodeState::Follower,
            current_term: 0,
            voted_for: None,
            log: HashMap::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
            election_timeout: Duration::from_millis(thread_rng().gen_range(1500..=3000)),
            heartbeat_interval: Duration::from_millis(500),
            last_heartbeat: None,
        }
    }

    fn start_election(&mut self){
        self.current_term += 1;
        self.voted_for = Some.(self.id);
        self.votes_received.insert(self.id);
        self.state = NodeState::Candidate;

        let request_vote_args = RequestVote{
            term: self.current_term,
            candidate_id: self.id,
            last_log_index: self.log.len() as u64,
            last_log_term: self.current_term,
        };
    }

    
}