import * as anchor from "@project-serum/anchor";
import {
	Transaction,
	PublicKey,
	SystemProgram,
	TransactionInstruction,
} from "@solana/web3.js";
import { Sigorg } from "../../types";
import { pda } from "../../utils";

export const addInstruction = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey,
	proposalIndex: number,
	instructionIndex: number,
	ix: TransactionInstruction
): Promise<Transaction> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);
	const proposal = await pda.proposal(program, organization, proposalIndex);
	const instruction = await pda.instruction(
		program,
		proposal,
		instructionIndex
	);

	// Initializer token account
	const initializerTeamTokenAccount = await pda.ata(
		wallet.publicKey,
		organizationData.teamTokenMint
	);
	const teamTokenAccount =
		await program.provider.connection.getParsedAccountInfo(
			initializerTeamTokenAccount
		);
	let initializerTokenAccount: PublicKey;
	if (
		teamTokenAccount !== null &&
		teamTokenAccount.value !== null &&
		"parsed" in teamTokenAccount.value.data &&
		parseInt(teamTokenAccount.value.data.parsed.info.tokenAmount.amount) >=
			1 &&
		teamTokenAccount.value.data.parsed.info.state === "frozen"
	) {
		initializerTokenAccount = initializerTeamTokenAccount;
	} else {
		initializerTokenAccount = await pda.ata(
			wallet.publicKey,
			organizationData.memberTokenMint
		);
	}

	const tx = await program.methods
		.addInstruction(
			new anchor.BN(proposalIndex),
			ix.programId,
			ix.keys,
			ix.data
		)
		.accounts({
			initializer: wallet.publicKey,
			initializerTokenAccount: initializerTokenAccount,
			organization: organization,
			memberTokenMint: organizationData.memberTokenMint,
			proposal: proposal,
			instruction: instruction,
			systemProgram: SystemProgram.programId,
		})
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
