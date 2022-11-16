import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import { TransactionInstruction, PublicKey } from "@solana/web3.js";

export const editOrganizationSettings = async (
	program: anchor.Program<Sigorg>,
	organization: PublicKey,
	defaultVotingTime: number,
	securityVotingTime: number,
	dividendClaimingTime: number,
	requiredTokenToSubmitProposal: number,
	voteReward: number
): Promise<TransactionInstruction> => {
	const ix = await program.methods
		.editOrganizationSettings(
			new anchor.BN(defaultVotingTime),
			new anchor.BN(securityVotingTime),
			new anchor.BN(dividendClaimingTime),
			requiredTokenToSubmitProposal,
			voteReward
		)
		.accounts({
			organization: organization,
		})
		.signers([])
		.instruction();

	return ix;
};
