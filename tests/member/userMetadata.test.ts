import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sigorg } from "../../target/types/sigorg";
import {
	Keypair,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";

import { editUserMetadata, pda } from "../../app/api";
import { createAndExecuteProposal } from "../utils/createAndExecuteProposal";

import * as assert from "assert";

describe("User Metadata", () => {
	anchor.setProvider(anchor.AnchorProvider.env());
	const program = anchor.workspace.Sigorg as Program<Sigorg>;
	const wallet = new anchor.Wallet(Keypair.generate());

	it("Init", async () => {
		// Airdrop
		const airdropSignature =
			await program.provider.connection.requestAirdrop(
				wallet.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 10
			);

		await program.provider.connection.confirmTransaction(airdropSignature);
	});

	it("Init Metadata", async () => {
		const name = "Test";
		const uri = "https://example.com";

		const tx = await editUserMetadata(program, wallet, name, uri);

		const signedTx = await wallet.signTransaction(tx);
		const signature = await program.provider.connection.sendRawTransaction(
			signedTx.serialize()
		);
		await program.provider.connection.confirmTransaction(signature);

		// Check
		const metadata = await pda.metadata(program, wallet.publicKey);
		const metadataAccount = await program.account.metadata.fetch(metadata);

		assert.equal(
			metadataAccount.target.toBase58(),
			wallet.publicKey.toBase58()
		);
		assert.ok(metadataAccount.targetType["user"] !== undefined);
		assert.equal(metadataAccount.name, name);
		assert.equal(metadataAccount.symbol, null);
		assert.equal(metadataAccount.uri, uri);
		assert.ok(metadataAccount.lastChange.toNumber() > 0);
	});

	it("Edit Metadata", async () => {
		const name = "Test 2";
		const uri = "https://example2.com";

		const tx = await editUserMetadata(program, wallet, name, uri);

		const signedTx = await wallet.signTransaction(tx);
		const signature = await program.provider.connection.sendRawTransaction(
			signedTx.serialize()
		);
		await program.provider.connection.confirmTransaction(signature);

		// Check
		const metadata = await pda.metadata(program, wallet.publicKey);
		const metadataAccount = await program.account.metadata.fetch(metadata);

		assert.equal(
			metadataAccount.target.toBase58(),
			wallet.publicKey.toBase58()
		);
		assert.ok(metadataAccount.targetType["user"] !== undefined);
		assert.equal(metadataAccount.name, name);
		assert.equal(metadataAccount.symbol, null);
		assert.equal(metadataAccount.uri, uri);
		assert.ok(metadataAccount.lastChange.toNumber() > 0);
	});

	it("Metadata string length", async () => {
		const name = "Test";
		const uri =
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis rhoncus massa non dolor suscipit laoreet. Etiam semper leo mi, ut maximus odio porttitor. ";

		const tx = await editUserMetadata(program, wallet, name, uri);

		const signedTx = await wallet.signTransaction(tx);

		let test: boolean;
		try {
			const signature =
				await program.provider.connection.sendRawTransaction(
					signedTx.serialize()
				);
			await program.provider.connection.confirmTransaction(signature);
			test = false;
		} catch {
			test = true;
		}

		assert.ok(test);
	});
});
