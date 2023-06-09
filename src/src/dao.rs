use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use thiserror::Error;

// Define custom error types for the DAO Framework
#[derive(Error, Debug)]
pub enum DaoError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Proposal not found")]
    ProposalNotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub address: String,
    pub voting_power: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub amount: f64,
    pub beneficiary: String,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dao {
    pub members: HashMap<String, Member>,
    pub proposals: HashMap<u32, Proposal>,
    pub funds: f64,
}

impl Dao {
    pub fn new() -> Self {
        Dao {
            members: HashMap::new(),
            proposals: HashMap::new(),
            funds: 0.0,
        }
    }

    pub fn add_member(&mut self, address: String, voting_power: u32) {
        self.members.insert(address, Member { address: address.clone(), voting_power });
    }
}