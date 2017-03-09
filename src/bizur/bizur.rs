use std::collections::HashMap;
use std::option::Option;

struct Node {
    
}

struct Server {
    id: usize,
    elect_id: usize,
    voted_elect_id: usize,
    own_voted: Option<usize, usize>,
    leader: Option<usize>,
    nodes: HashMap<usize, Node>,
}

enum Message {
    REQUESTVOTE(),
    ACKVOTE(),
    NACKVOTE(),
}

struct RequestVote {
    elect_id: usize,
    leader: usize,
}

struct AckVote {
    id: usize,
    elect_id: usize,
}

struct NackVote {
    id: usize,
    elect_id: usize,
}

impl Server {
    fn start_election(&mut self) {
        self.elect_id = self.elect_id + 1;
        self.leader = None;
        self.own_voted = Some(self.slect_id);

        self.fire_ownvote_timeout();
    }

    fn please_vote(&mut self) {
        
    }

    fn major(&mut self) -> usize {
        return self.nodes.len() / 2 + 1;
    }

    fn process_please_voet(&mut self, req: &RequestVote) {
        if req.elect_id > self.voted_elect_id {
            self.voted_elect_id = req.elect_id;
            self.elect_id = req.elect_id;
            self.leader = Some(req.leader);
            self.ack_vote();
        } else if req.elect_id == self.voted_elect_id && self.leader.unwrap() == req.leader {
            self.ack_vote();
        } else {
            self.nack_vote();
        }
    }
    fn ack_vote(&mut self) {
        
    }

    fn process_ack_vote(&mut self, ack: &AckVote) {
        if self.leader.is_none() and self.own_vote.is_some() {
            let (elect_id, count) = self.own_vote.take().unwrap();
            if elect_id == ack.elect_id {
                let count = count + 1;
                self.own_voted = Some((elect_id, count));
                if count >= self.major() {
                    self.leader = Some(self.id);
                    self.own_voted = None;
                }
            }
    } 

    fn nack_vote(&mut self) {
        
    }

    fn process_nack_vote(&mut self, nack: &NackVote) {
        
    }

    fn fire_noleader_timeout(&mut self) {
        
    }

    fn fire_ownvote_timeout(&mut self) {
        
    }

    fn process_noleader_timeout(&mut self) {
        if self.leader.is_none() && self.own_vote.is_none() {
            self.start_election();
        } 
    }

    fn process_ownvote_timeout(&mut self) {
        if self.leader.is_some() || self.own_vote.is_none() {
            return ;
        } 

        self.start_election();
    }
