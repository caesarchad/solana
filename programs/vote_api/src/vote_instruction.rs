use crate::id;
use serde_derive::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::{AccountMeta, Instruction};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Vote {
    // TODO: add signature of the state here as well
    /// A vote for height slot
    pub slot: u64,
}

impl Vote {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum VoteInstruction {
    /// Initialize the VoteState for this `vote account`
    /// * Instruction::keys[0] - the new "vote account" to be associated with the delegate
    InitializeAccount,
    /// `Delegate` or `Assign` a vote account to a particular node
    DelegateStake(Pubkey),
    /// Authorize a voter to send signed votes.
    AuthorizeVoter(Pubkey),
    Vote(Vote),
    /// Clear the credits in the vote account
    /// * Transaction::keys[0] - the "vote account"
    ClearCredits,
}

impl VoteInstruction {
    pub fn new_clear_credits(vote_id: &Pubkey) -> Instruction {
        let account_metas = vec![AccountMeta::new(*vote_id, true)];
        Instruction::new(id(), &VoteInstruction::ClearCredits, account_metas)
    }
    pub fn new_delegate_stake(vote_id: &Pubkey, delegate_id: &Pubkey) -> Instruction {
        let account_metas = vec![AccountMeta::new(*vote_id, true)];
        Instruction::new(
            id(),
            &VoteInstruction::DelegateStake(*delegate_id),
            account_metas,
        )
    }
    pub fn new_authorize_voter(vote_id: &Pubkey, authorized_voter_id: &Pubkey) -> Instruction {
        let account_metas = vec![AccountMeta::new(*vote_id, true)];
        Instruction::new(
            id(),
            &VoteInstruction::AuthorizeVoter(*authorized_voter_id),
            account_metas,
        )
    }
    pub fn new_initialize_account(vote_id: &Pubkey) -> Instruction {
        let account_metas = vec![AccountMeta::new(*vote_id, false)];
        Instruction::new(id(), &VoteInstruction::InitializeAccount, account_metas)
    }
    pub fn new_vote(vote_id: &Pubkey, vote: Vote) -> Instruction {
        let account_metas = vec![AccountMeta::new(*vote_id, true)];
        Instruction::new(id(), &VoteInstruction::Vote(vote), account_metas)
    }
}
