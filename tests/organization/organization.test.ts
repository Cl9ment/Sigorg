import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sigorg } from "../../target/types/sigorg";
import { Keypair, PublicKey } from "@solana/web3.js";

import {
	createOrganization,
	pda,
	editOrganizationSettings,
	initFreezer,
} from "../../app/api";

import * as assert from "assert";
import { createAndExecuteProposal } from "../utils/createAndExecuteProposal";

describe("Organization", () => {
	anchor.setProvider(anchor.AnchorProvider.env());
	const program = anchor.workspace.Sigorg as Program<Sigorg>;
	const wallet = new anchor.Wallet(Keypair.generate());

	let organization: PublicKey;

	it("Init", async () => {
		const airdropSignature =
			await program.provider.connection.requestAirdrop(
				wallet.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 10
			);

		await program.provider.connection.confirmTransaction(airdropSignature);
	});

	it("Create organization", async () => {
		const organizationTx = await createOrganization(program, wallet);

		// send tx
		const signedTx = await wallet.signTransaction(organizationTx.tx);
		const signature = await program.provider.connection.sendRawTransaction(
			signedTx.serialize()
		);
		await program.provider.connection.confirmTransaction(signature);

		// Checks
		const data = await program.account.organization.fetch(
			organizationTx.organization
		);
		const organizationPda = await PublicKey.findProgramAddress(
			[Buffer.from("organization"), data.memberTokenMint.toBuffer()],
			program.programId
		);

		assert.equal(data.nonce, organizationPda[1]);
		assert.equal(data.initializer.toBase58(), wallet.publicKey.toBase58());
		assert.equal(
			data.nativeVault.toBase58(),
			(
				await pda.organizationVault(
					program,
					organizationTx.organization
				)
			).toBase58()
		);
		assert.equal(data.proposalIndex, 0);
		assert.equal(data.protocolIndex, 0);
		assert.equal(data.dividendIndex, 0);

		organization = organizationTx.organization;

		// Init freezer
		const initFreezerTx = await initFreezer(program, wallet, organization);
		const initFreezerSignedTx = await wallet.signTransaction(initFreezerTx);
		const initFreezerSignature =
			await program.provider.connection.sendRawTransaction(
				initFreezerSignedTx.serialize()
			);
		await program.provider.connection.confirmTransaction(
			initFreezerSignature
		);
	});

	it("Edit settings", async () => {
		const defaultVotingTime = 666;
		const securityVotingTime = 333;
		const dividendClaimingTime = 999;
		const requiredTokenToSubmitProposal = 1;
		const voteReward = 0.1;

		const ix = await editOrganizationSettings(
			program,
			organization,
			defaultVotingTime,
			securityVotingTime,
			dividendClaimingTime,
			requiredTokenToSubmitProposal,
			voteReward
		);
		const _proposalIndex = await createAndExecuteProposal(
			program,
			wallet,
			organization,
			[ix]
		);

		// Check
		const organizationAccount = await program.account.organization.fetch(
			organization
		);

		assert.equal(
			organizationAccount.defaultVotingTime.toNumber(),
			defaultVotingTime
		);
		assert.equal(
			organizationAccount.securityVotingTime.toNumber(),
			securityVotingTime
		);
		assert.equal(
			organizationAccount.dividendClaimingTime.toNumber(),
			dividendClaimingTime
		);
		assert.equal(
			organizationAccount.requiredTokenToSubmitProposal,
			requiredTokenToSubmitProposal
		);
		assert.equal(organizationAccount.voteReward, voteReward);
	});
});
