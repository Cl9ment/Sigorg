import * as anchor from "@project-serum/anchor";
import { Transaction, PublicKey } from "@solana/web3.js";
import { Sigorg } from "../../types";
import { pda } from "../../utils";

export const submitProposal = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey,
	proposalIndex: number,
	executionTimestamp: number,
	isSecurityProposal: boolean = false,
	uri?: string
): Promise<Transaction> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);
	const proposal = await pda.proposal(program, organization, proposalIndex);

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
		.submitProposal(
			new anchor.BN(proposalIndex),
			new anchor.BN(executionTimestamp),
			isSecurityProposal,
			uri ? uri : null
		)
		.accounts({
			initializer: wallet.publicKey,
			initializerTokenAccount: initializerTokenAccount,
			organization: organization,
			memberTokenMint: organizationData.memberTokenMint,
			proposal: proposal,
		})
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
