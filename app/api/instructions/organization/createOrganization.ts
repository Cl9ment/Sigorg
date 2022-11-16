import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import {
	Transaction,
	PublicKey,
	Keypair,
	SystemProgram,
} from "@solana/web3.js";
import {
	TOKEN_PROGRAM_ID,
	ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { pda } from "../../utils";

export const createOrganization = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet
): Promise<{
	tx: Transaction;
	organization: PublicKey;
}> => {
	const memberTokenMint = Keypair.generate();
	const teamTokenMint = Keypair.generate();

	const organization = await pda.organization(
		program,
		memberTokenMint.publicKey
	);
	const organizationVault = await pda.organizationVault(
		program,
		organization
	);

	const initializerMemberAccount = await pda.ata(
		wallet.publicKey,
		memberTokenMint.publicKey
	);

	const tx = await program.methods
		.createOrganization()
		.accounts({
			initializer: wallet.publicKey,
			memberTokenMint: memberTokenMint.publicKey,
			teamTokenMint: teamTokenMint.publicKey,
			organization: organization,
			organizationVault: organizationVault,
			initializerMemberAccount: initializerMemberAccount,
			systemProgram: SystemProgram.programId,
			tokenProgram: TOKEN_PROGRAM_ID,
			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
			rent: anchor.web3.SYSVAR_RENT_PUBKEY,
		})
		.signers([memberTokenMint, teamTokenMint])
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;
	tx.partialSign(memberTokenMint);
	tx.partialSign(teamTokenMint);

	return {
		tx: tx,
		organization: organization,
	};
};
