import * as anchor from "@project-serum/anchor";
import { Transaction, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Sigorg } from "../../types";
import { pda } from "../../utils";

export const voteProposal = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey,
	proposalIndex: number,
	approval: boolean,
	note?: string
): Promise<Transaction> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);

	const proposal = await pda.proposal(program, organization, proposalIndex);
	const voterMemberTokenAccount = await pda.ata(
		wallet.publicKey,
		organizationData.memberTokenMint
	);
	const vote = await pda.vote(program, proposal, voterMemberTokenAccount);
	const freezer = await pda.freezer(program, voterMemberTokenAccount);

	const tx = await program.methods
		.vote(new anchor.BN(proposalIndex), approval, note ? note : null)
		.accounts({
			voter: wallet.publicKey,
			voterMemberTokenAccount: voterMemberTokenAccount,
			organization: organization,
			memberTokenMint: organizationData.memberTokenMint,
			proposal: proposal,
			vote: vote,
			freezer: freezer,
			systemProgram: SystemProgram.programId,
			tokenProgram: TOKEN_PROGRAM_ID,
		})
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
