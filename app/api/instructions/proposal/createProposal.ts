import * as anchor from "@project-serum/anchor";
import { Transaction, PublicKey, SystemProgram } from "@solana/web3.js";
import { Sigorg } from "../../types";
import { pda } from "../../utils";

export const createProposal = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey
): Promise<{
	tx: Transaction;
	proposalIndex: number;
}> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);

	const index = organizationData.proposalIndex.toNumber();
	const proposal = await pda.proposal(program, organization, index);

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
		.createProposal()
		.accounts({
			initializer: wallet.publicKey,
			initializerTokenAccount: initializerTokenAccount,
			organization: organization,
			memberTokenMint: organizationData.memberTokenMint,
			proposal: proposal,
			feeCollector: pda.feeCollector(),
			systemProgram: SystemProgram.programId,
		})
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return { tx: tx, proposalIndex: index };
};
