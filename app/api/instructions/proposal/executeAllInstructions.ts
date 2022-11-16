import * as anchor from "@project-serum/anchor";
import { Transaction, PublicKey, SystemProgram } from "@solana/web3.js";
import { Sigorg } from "../../types";
import { pda } from "../../utils";

export const executeAllInstructions = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey,
	proposalIndex: number
): Promise<Transaction> => {
	const organizationVault = await pda.organizationVault(
		program,
		organization
	);
	const proposal = await pda.proposal(program, organization, proposalIndex);

	const proposalData = await program.account.proposal.fetch(proposal);

	const tx = new Transaction();
	for (
		let i = proposalData.executedInstructions.toNumber();
		i < proposalData.instructionsAmount.toNumber();
		i++
	) {
		const instruction = await pda.instruction(program, proposal, i);

		const instructionData = await program.account.instruction.fetch(
			instruction
		);

		// @ts-ignore
		let accounts: any[] = instructionData.accounts.map((account) => {
			account.isSigner = false;
			return account;
		});
		accounts.push({
			pubkey: instructionData.programId,
			isWritable: false,
			isSigner: false,
		});

		const txI = await program.methods
			.executeProposalInstruction(new anchor.BN(proposalIndex))
			.remainingAccounts(accounts)
			.accounts({
				initializer: proposalData.initializer,
				organization: organization,
				organizationVault: organizationVault,
				proposal: proposal,
				instruction: instruction,
				systemProgram: SystemProgram.programId,
			})
			.transaction();

		tx.add(txI);
	}

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
