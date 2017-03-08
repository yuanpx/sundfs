use std::ops::Range;
use std::option::Option;
use std::collections::HashMap;


enum Role {
    LEADER,
    FOLLOWER,
    CANDIDATE,
}

enum Phase {
    
}

struct Node {
    
}

struct Config {
    heartbeat: usize,
    log_idex: usize,
    
}

struct Raft {
    id: usize,
    leader_id: Option<usize>,
    nodes: HashMap<usize, Node>,
    role: Role,
    term: usize,
}

enum Message {
    REQUESTVOTE(),
    APPENDENTRIES(),
    HEARTBEAT(),
}

struct RequestVote{
     
}



impl Process for Raft {
    fn process_init(&mut self, service: &mut Service) {
        
    }
    fn process_event(&mut self, service: &mut Service, ev: NetEvent)
    {

    }
    
}

impl Raft {
    fn major(&self) -> usize {
        self.nodes.len() / 2 + 1  
    }

    fn send_requestvote() {
    }

    fn send_appendentries() {
        
    }

    fn send_message() {
        
    }

    fn process_appendentries() {
        
    }

    fn process_appendentries() {
        
    }
}
