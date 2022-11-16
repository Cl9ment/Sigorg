import { PublicKey } from "@solana/web3.js";
import {
	TOKEN_PROGRAM_ID,
	ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Sigorg } from "../types";
import * as anchor from "@project-serum/anchor";
import { BN } from "@project-serum/anchor";

export const ata = async (
	authority: PublicKey,
	mint: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[authority.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
		ASSOCIATED_TOKEN_PROGRAM_ID
	);

	return pda[0];
};

export const organization = async (
	program: anchor.Program<Sigorg>,
	memberTokenMint: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[Buffer.from("organization"), memberTokenMint.toBuffer()],
		program.programId
	);

	return pda[0];
};

export const organizationVault = async (
	program: anchor.Program<Sigorg>,
	organization: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[Buffer.from("vault"), organization.toBuffer()],
		program.programId
	);

	return pda[0];
};

export const proposal = async (
	program: anchor.Program<Sigorg>,
	organization: PublicKey,
	proposalIndex: number
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[
			Buffer.from("proposal"),
			organization.toBuffer(),
			new BN(proposalIndex).toArrayLike(Buffer, "be", 8),
		],
		program.programId
	);

	return pda[0];
};

export const instruction = async (
	program: anchor.Program<Sigorg>,
	proposal: PublicKey,
	instructionIndex: number
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[
			Buffer.from("instruction"),
			proposal.toBuffer(),
			new BN(instructionIndex).toArrayLike(Buffer, "be", 8),
		],
		program.programId
	);

	return pda[0];
};

export const feeCollector = (): PublicKey => {
	return new PublicKey("CLjksmsXLEYD8J8LuRdKdP2ASyN9P7PB6aeD8vceEGvY");
};

export const vote = async (
	program: anchor.Program<Sigorg>,
	proposal: PublicKey,
	voterMemberTokenAccount: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[
			Buffer.from("vote"),
			proposal.toBuffer(),
			voterMemberTokenAccount.toBuffer(),
		],
		program.programId
	);

	return pda[0];
};

export const freezer = async (
	program: anchor.Program<Sigorg>,
	memberTokenAccount: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[Buffer.from("freezer"), memberTokenAccount.toBuffer()],
		program.programId
	);

	return pda[0];
};

export const metadata = async (
	program: anchor.Program<Sigorg>,
	target: PublicKey
): Promise<PublicKey> => {
	let pda = await PublicKey.findProgramAddress(
		[Buffer.from("metadata"), target.toBuffer()],
		program.programId
	);

	return pda[0];
};
