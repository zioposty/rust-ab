use uid::Id as IdT;
use crate::agent::Agent;
// use std::cmp::Eq;
use std::hash::{Hash};
use std::fmt;
use crate::simstate::SimState;
use std::clone::Clone;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AgentImpl<A: Agent + Clone + Copy + Hash + Eq>{
    pub id: IdT<A>,
    pub agent: A,
    pub repeating: bool,
}

impl<A: Agent + Clone + Copy + Hash + Eq> AgentImpl<A> {
    pub fn new(agent: A) -> AgentImpl<A>
        where A: Agent
    {
        AgentImpl {
            id: IdT::new(),
            agent: agent,
            repeating: false,
        }
    }

    pub fn step(self, simstate: &SimState<A>) {
        self.agent.step(simstate);
    }

    pub fn id(self) -> usize {
        self.id.get()
    }
}

impl<A: Agent + Clone + Copy + Hash + Eq> fmt::Display for AgentImpl<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.repeating)
    }
}
