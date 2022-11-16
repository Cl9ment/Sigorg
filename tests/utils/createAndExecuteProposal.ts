import * as anchor from "@project-serum/anchor";
import {
	Transaction,
	PublicKey,
	SystemProgram,
	TransactionInstruction,
} from "@solana/web3.js";
import {
	Sigorg,
	createProposal,
	addInstruction,
	submitProposal,
	voteProposal,
	executeAllInstructions,
	pda,
} from "../../app/api";
import * as assert from "assert";

export const createAndExecuteProposal = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey,
	instructions: TransactionInstruction[]
): Promise<number> => {
	let transactions: Transaction[] = [];

	const createProposalTx = await createProposal(
		program,
		wallet,
		organization
	);
	transactions.push(createProposalTx.tx);

	for (let i = 0; i < instructions.length; i++) {
		const addInstructionTx = await addInstruction(
			program,
			wallet,
			organization,
			createProposalTx.proposalIndex,
			i,
			instructions[i]
		);
		transactions.push(addInstructionTx);
	}

	const submitProposalTx = await submitProposal(
		program,
		wallet,
		organization,
		createProposalTx.proposalIndex,
		0,
		false,
		"https://example.com/id"
	);
	transactions.push(submitProposalTx);

	const voteProposalTx = await voteProposal(
		program,
		wallet,
		organization,
		createProposalTx.proposalIndex,
		true,
		"https://example.com/id"
	);
	transactions.push(voteProposalTx);

	// Send txs
	const signedTxs = await wallet.signAllTransactions(transactions);
	for (let i = 0; i < signedTxs.length; i++) {
		const signature = await program.provider.connection.sendRawTransaction(
			signedTxs[i].serialize()
			/* {
				skipPreflight: true,
			} */
		);

		await program.provider.connection.confirmTransaction(signature);
	}

	// Execute proposal
	const executeAllInstructionsTx = await executeAllInstructions(
		program,
		wallet,
		organization,
		createProposalTx.proposalIndex
	);
	const signedTx = await wallet.signTransaction(executeAllInstructionsTx);
	const signatureTx = await program.provider.connection.sendRawTransaction(
		signedTx.serialize(),
		{
			skipPreflight: true,
		}
	);
	const result = await program.provider.connection.confirmTransaction(
		signatureTx
	);

	// Check proposal did execute
	const proposal = await pda.proposal(
		program,
		organization,
		createProposalTx.proposalIndex
	);
	const proposalData = await program.account.proposal.fetch(proposal);
	assert.ok(proposalData.status["finalized"] !== undefined);

	return createProposalTx.proposalIndex;
};
