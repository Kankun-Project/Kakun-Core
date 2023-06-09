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