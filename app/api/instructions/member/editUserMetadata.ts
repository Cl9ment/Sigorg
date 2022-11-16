import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import { Transaction, SystemProgram } from "@solana/web3.js";
import { pda } from "../../utils";

export const editUserMetadata = async (
	program: anchor.Program<Sigorg>,
	wallet: anchor.Wallet,
	name?: string,
	uri?: string
): Promise<Transaction> => {
	const metadata = await pda.metadata(program, wallet.publicKey);

	let metadataAccount = undefined;
	try {
		metadataAccount = await program.account.metadata.fetch(metadata);
	} catch {}

	let tx: Transaction;

	if (metadataAccount === undefined) {
		tx = await program.methods
			.initUserMetadata(name ? name : null, uri ? uri : null)
			.accounts({
				user: wallet.publicKey,
				metadata: metadata,
				systemProgram: SystemProgram.programId,
			})
			.signers([])
			.transaction();
	} else {
		tx = await program.methods
			.editUserMetadata(name ? name : null, uri ? uri : null)
			.accounts({
				user: wallet.publicKey,
				metadata: metadata,
				systemProgram: SystemProgram.programId,
			})
			.signers([])
			.transaction();
	}

	tx.feePayer = wallet.publicKey;
	tx.recentBlockhash = (
		await program.provider.connection.getLatestBlockhash()
	).blockhash;

	return tx;
};
