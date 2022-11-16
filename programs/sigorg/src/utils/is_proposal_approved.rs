use crate::states::*;
use anchor_lang::prelude::*;

pub fn is_proposal_approved(proposal: &mut Account<Proposal>) -> bool {
    if proposal.status == ProposalStatus::Approved {
        return true;
    }

    if proposal.approval > proposal.threshold {
        proposal.status = ProposalStatus::Approved;
        return true;
    }

    if proposal.vote_end_timestamp < Clock::get().unwrap().unix_timestamp {
        proposal.status = ProposalStatus::Declined;
    }

    return false;
}
