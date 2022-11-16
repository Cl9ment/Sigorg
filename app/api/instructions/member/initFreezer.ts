import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import { Transaction, PublicKey, SystemProgram } from "@solana/web3.js";
import { pda } from "../../utils";

export const initFreezer = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	organization: PublicKey
): Promise<Transaction> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);

	const memberTokenAccount = await pda.ata(
		wallet.publicKey,
		organizationData.memberTokenMint
	);

	const freezer = await pda.freezer(program, memberTokenAccount);

	const tx = await program.methods
		.initFreezer()
		.accounts({
			member: wallet.publicKey,
			memberTokenAccount: memberTokenAccount,
			organization: organization,
			freezer: freezer,
			systemProgram: SystemProgram.programId,
		})
		.signers([])
		.transaction();

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
