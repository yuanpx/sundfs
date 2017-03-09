use storage::logic::Process;
use storage::logic::Service;
use storage::nio::NetEvent;
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
    follower_heartbeat_id: usize,
    follower_heartbeat_timeout: usize,
    leader_heartbeat_id: usize,
    leader_heartbeat_timeout: usize,
    candidate_process_id: usize,
    candidate_process__timeout: usize,
}

struct Raft {
    id: usize,
    leader_id: Option<usize>,
    nodes: HashMap<usize, Node>,
    role: Role,
    term: usize,
    vote: (usize, usize),
    receive_leader_heartbeat: bool,
    conf: Config,
}

enum Log {
    
}

enum Message {
    REQUESTVOTE(),
    APPENDENTRY(),
    HEARTBEAT(),
}

struct RequestVote{
    term: usize,
    leader_id: usize,
}

struct ResponseVote {
    term: usize,
}

struct AppendEntry{
    term: usize,
}





impl Process for Raft {
    fn process_init(&mut self, service: &mut Service) {
        
    }
    fn process_event(&mut self, service: &mut Service, ev: NetEvent) {
        match ev {
            NetEvent::CONNECTED((event,id)) => {
                println!("connected id: {}", id);
            },
            NetEvent::DISCONNECTED((event,id))=> {
                println!("disconnected id: {}", id);
                
            },
            NetEvent::MESSAGE((id, buf)) => {
                println!("message id: {}", id);
            },
            NetEvent::TIMEOUT(event) => {
                let follower_heartbeat_id = self.conf.follower_heartbeat_id;
                let leader_heartbeat_id = self.conf.follower_heartbeat_id;
                let candidate_process_id = self.conf.candidate_process_id;

                if event == follower_heartbeat_id {
                    self.process_follower_heartbeat_timeout(service);
                } else if event == leader_heartbeat_id {
                    self.process_leader_heartbeat_timeout(service);
                } else if event == candidate_process_id {
                    self.process_candidate_requestvote_timeout(service);
                }
            }
        }
    } 
}

impl Raft {
    fn major(&self) -> usize {
        self.nodes.len() / 2 + 1  
    }

    fn send_requestvote(&mut self) {
    }

    fn send_appendentries(&mut self) {
        
    }

    fn send_leader_heartbeat(&mut self) {
        
    }

    fn begin_leader_heartbeat_timeout(&self, service: &mut Service) {
        service.timeout(self.conf.leader_heartbeat_id, self.conf.leader_heartbeat_timeout);
    }

    fn begin_follower_heartbeat_timeout(&self, service:&mut Service) {
        service.timeout(self.conf.follower_heartbeat_id, self.conf.follower_heartbeat_timeout);
    }

    fn begin_candidate_requestvote_timeout(&self, service:&mut Service) {
        service.timeout(self.conf.candidate_process_id, self.conf.candidate_process__timeout);
    }

    fn process_leader_heartbeat_timeout(&mut self, service: &mut Service) {

        match self.role {
            Role::LEADER => {
                self.begin_leader_heartbeat_timeout(service);
            },
            _ => {},
        };

    }

    fn process_follower_heartbeat_timeout(&mut self, service: &mut Service) {
        if self.receive_leader_heartbeat == false {
            self.become_candidate();
        } else {
            self.begin_follower_heartbeat_timeout(service);
        }
    }

    fn process_candidate_requestvote_timeout(&mut self, service: &mut Service) {

        match self.role {
            Role::CANDIDATE => {
                self.become_candidate();
            },
            _ => {
            },
        };
    }

    fn process_appendentries(&mut self, entry: AppendEntry) {
        
    }

    fn process_leader_heartbeat(&mut self) {
        
    }

    fn process_requestvote(&mut self, request: RequestVote) {
        
    }

    fn become_follower(&mut self) {
        
    }

    fn become_candidate(&mut self) {
    }

    fn become_leader(&mut self) {
    }

}
