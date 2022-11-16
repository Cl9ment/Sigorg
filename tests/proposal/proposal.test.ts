import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sigorg } from "../../target/types/sigorg";
import {
	Keypair,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";

import {
	createOrganization,
	pda,
	createProposal,
	addInstruction,
	submitProposal,
	voteProposal,
	executeAllInstructions,
	initFreezer,
} from "../../app/api";
import { createAndExecuteProposal } from "../utils/createAndExecuteProposal";

import * as assert from "assert";

describe("Proposal", () => {
	anchor.setProvider(anchor.AnchorProvider.env());
	const program = anchor.workspace.Sigorg as Program<Sigorg>;
	const wallet = new anchor.Wallet(Keypair.generate());

	let organization: PublicKey;
	let proposalIndex: number;
	const toKeypair = Keypair.generate();

	it("Init", async () => {
		// Airdrop
		const airdropSignature =
			await program.provider.connection.requestAirdrop(
				wallet.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 10
			);

		await program.provider.connection.confirmTransaction(airdropSignature);

		// Create organization
		const createOrganizationTx = await createOrganization(program, wallet);
		const createOrganizationSignedTx = await wallet.signTransaction(
			createOrganizationTx.tx
		);
		const createOrganizationSignature =
			await program.provider.connection.sendRawTransaction(
				createOrganizationSignedTx.serialize()
			);
		await program.provider.connection.confirmTransaction(
			createOrganizationSignature
		);
		organization = createOrganizationTx.organization;

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

	it("Create and submit proposal", async () => {
		const airdropSignature =
			await program.provider.connection.requestAirdrop(
				toKeypair.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 1
			);
		await program.provider.connection.confirmTransaction(airdropSignature);

		let txs: Transaction[] = [];

		const proposal = await createProposal(program, wallet, organization);
		proposalIndex = proposal.proposalIndex;
		txs.push(proposal.tx);

		const ix0 = SystemProgram.transfer({
			fromPubkey: await pda.organizationVault(program, organization),
			toPubkey: toKeypair.publicKey,
			lamports: 3,
		});
		const tx0 = await addInstruction(
			program,
			wallet,
			organization,
			proposal.proposalIndex,
			0,
			ix0
		);
		txs.push(tx0);

		const ix1 = SystemProgram.transfer({
			fromPubkey: await pda.organizationVault(program, organization),
			toPubkey: toKeypair.publicKey,
			lamports: 2,
		});
		const tx1 = await addInstruction(
			program,
			wallet,
			organization,
			proposal.proposalIndex,
			1,
			ix1
		);
		txs.push(tx1);

		const tx2 = await submitProposal(
			program,
			wallet,
			organization,
			proposal.proposalIndex,
			0
		);
		txs.push(tx2);

		let signature;
		const signedTxs = await wallet.signAllTransactions(txs);
		for (let i = 0; i < signedTxs.length; i++) {
			signature = await program.provider.connection.sendRawTransaction(
				signedTxs[i].serialize(),
				{
					skipPreflight: true,
				}
			);
			await program.provider.connection.confirmTransaction(signature);
		}

		// Checks
		const proposalKey = await pda.proposal(
			program,
			organization,
			proposalIndex
		);

		// Check proposal
		const proposalData = await program.account.proposal.fetch(proposalKey);
		const ix0Data = await program.account.instruction.fetch(
			await pda.instruction(program, proposalKey, 0)
		);
		const ix1Data = await program.account.instruction.fetch(
			await pda.instruction(program, proposalKey, 1)
		);

		//console.log(proposalData);
		//console.log(ix0Data);
		//console.log(ix1Data);

		// Check proposal
		assert.equal(
			proposalData.organization.toBase58(),
			organization.toBase58()
		);
		assert.equal(proposalData.index.toNumber(), 0);
		assert.equal(
			proposalData.initializer.toBase58(),
			wallet.publicKey.toBase58()
		);
		assert.equal(proposalData.instructionsAmount.toNumber(), 2);
		assert.equal(proposalData.executedInstructions.toNumber(), 0);
		assert.ok(proposalData.status["waitingApproval"] !== undefined);
		// @ts-ignore
		assert.equal(proposalData.approval.toNumber(), 0);
		assert.equal(proposalData.disapproval.toNumber(), 0);

		// Check ix0
		assert.equal(ix0Data.organization.toBase58(), organization.toBase58());
		assert.equal(ix0Data.proposal.toBase58(), proposalKey.toBase58());
		assert.equal(ix0Data.index.toNumber(), 0);
		assert.equal(ix0Data.didExecute, false);
		assert.equal(ix0Data.programId.toBase58(), ix0.programId.toBase58());

		// Check ix1
		assert.equal(ix1Data.organization.toBase58(), organization.toBase58());
		assert.equal(ix1Data.proposal.toBase58(), proposalKey.toBase58());
		assert.equal(ix1Data.index.toNumber(), 1);
		assert.equal(ix1Data.didExecute, false);
		assert.equal(ix1Data.programId.toBase58(), ix1.programId.toBase58());
	});

	it("Vote proposal", async () => {
		const organizationData = await program.account.organization.fetch(
			organization
		);
		const voterMemberTokenAccount = await pda.ata(
			wallet.publicKey,
			organizationData.memberTokenMint
		);
		const tokenAccountBefore =
			await program.provider.connection.getParsedAccountInfo(
				voterMemberTokenAccount
			);

		const isApproved = true;
		const note = "Hello";

		const tx = await voteProposal(
			program,
			wallet,
			organization,
			proposalIndex,
			isApproved,
			note
		);

		const signedTx = await wallet.signTransaction(tx);
		const signature = await program.provider.connection.sendRawTransaction(
			signedTx.serialize(),
			{
				skipPreflight: true,
			}
		);

		await program.provider.connection.confirmTransaction(signature);

		// Accounts
		const proposalKey = await pda.proposal(
			program,
			organization,
			proposalIndex
		);
		const voteData = await program.account.vote.fetch(
			await pda.vote(program, proposalKey, voterMemberTokenAccount)
		);
		const freezerData = await program.account.freezer.fetch(
			await pda.freezer(program, voterMemberTokenAccount)
		);
		const proposalData = await program.account.proposal.fetch(proposalKey);

		//console.log(voteData);
		//console.log(freezerData);
		//console.log(proposalData);

		// Check vote
		assert.equal(voteData.voter.toBase58(), wallet.publicKey.toBase58());
		assert.equal(voteData.organization.toBase58(), organization.toBase58());
		assert.equal(voteData.proposal.toBase58(), proposalKey.toBase58());
		assert.equal(voteData.approval, isApproved);
		assert.equal(voteData.uri, note);

		// Check freezer
		assert.equal(
			freezerData.organization.toBase58(),
			organization.toBase58()
		);
		assert.equal(
			freezerData.tokenMint.toBase58(),
			organizationData.memberTokenMint.toBase58()
		);
		assert.equal(
			freezerData.tokenAccount.toBase58(),
			voterMemberTokenAccount.toBase58()
		);
		const votingTime =
			freezerData.thawTimestamp.toNumber() -
			voteData.voteTimestamp.toNumber();
		assert.ok(
			votingTime + 5 > organizationData.defaultVotingTime.toNumber() &&
				votingTime - 5 < organizationData.defaultVotingTime.toNumber()
		);

		// Check proposal
		assert.ok(proposalData.status["approved"] !== undefined);

		// Check token account is frozen
		const tokenAccount =
			await program.provider.connection.getParsedAccountInfo(
				voterMemberTokenAccount
			);
		assert.ok(
			tokenAccount !== null &&
				tokenAccount.value !== null &&
				"parsed" in tokenAccount.value.data &&
				tokenAccount.value.data.parsed.info.state === "frozen"
		);
		assert.ok(
			tokenAccountBefore !== null &&
				tokenAccountBefore.value !== null &&
				"parsed" in tokenAccountBefore.value.data &&
				voteData.tokenAmount.toNumber() ===
					parseInt(
						tokenAccountBefore.value.data.parsed.info.tokenAmount
							.amount
					)
		);
		const amountAfterReward = Math.floor(
			parseInt(
				tokenAccountBefore.value.data.parsed.info.tokenAmount.amount
			) *
				(organizationData.voteReward + 1)
		);
		assert.ok(
			amountAfterReward - 1 <=
				parseInt(
					tokenAccount.value.data.parsed.info.tokenAmount.amount
				) &&
				amountAfterReward + 1 >=
					parseInt(
						tokenAccount.value.data.parsed.info.tokenAmount.amount
					)
		);
	});

	it("Execute proposal", async () => {
		const tx = await executeAllInstructions(
			program,
			wallet,
			organization,
			proposalIndex
		);

		const signedTx = await wallet.signTransaction(tx);
		const signature = await program.provider.connection.sendRawTransaction(
			signedTx.serialize(),
			{
				skipPreflight: true,
			}
		);

		await program.provider.connection.confirmTransaction(signature);

		// Checks
		const proposal = await pda.proposal(
			program,
			organization,
			proposalIndex
		);
		const ix0 = await pda.instruction(program, proposal, 0);
		const ix1 = await pda.instruction(program, proposal, 1);

		const proposalData = await program.account.proposal.fetch(proposal);
		const ix0Data = await program.account.instruction.fetch(ix0);
		const ix1Data = await program.account.instruction.fetch(ix1);

		//console.log(proposalData);
		//console.log(ix0Data);
		//console.log(ix1Data);

		// Check proposal
		assert.equal(
			proposalData.executedInstructions.toNumber(),
			proposalData.instructionsAmount.toNumber()
		);
		assert.ok(proposalData.status["finalized"] !== undefined);

		// Check ix0
		assert.equal(ix0Data.didExecute, true);

		// Check ix1
		assert.equal(ix1Data.didExecute, true);

		const balance = await program.provider.connection.getBalance(
			toKeypair.publicKey
		);
		assert.equal(balance, 1000000005);
		//console.log(balance);
	});

	it("utils.createAndExecuteProposal", async () => {
		const toKeypair2 = Keypair.generate();

		const airdropSignature =
			await program.provider.connection.requestAirdrop(
				toKeypair2.publicKey,
				anchor.web3.LAMPORTS_PER_SOL * 1
			);
		await program.provider.connection.confirmTransaction(airdropSignature);

		const ix = SystemProgram.transfer({
			fromPubkey: await pda.organizationVault(program, organization),
			toPubkey: toKeypair2.publicKey,
			lamports: 9,
		});

		const _proposalIndex = await createAndExecuteProposal(
			program,
			wallet,
			organization,
			[ix]
		);

		// Check
		const balance = await program.provider.connection.getBalance(
			toKeypair2.publicKey
		);
		assert.equal(balance, 1000000009);
	});
});
