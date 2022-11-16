import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import {
	TransactionInstruction,
	PublicKey,
	SystemProgram,
} from "@solana/web3.js";
import {
	TOKEN_PROGRAM_ID,
	ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { pda } from "../../utils";

export const addTeamMember = async (
	program: anchor.Program<Sigorg>,
	organization: PublicKey,
	receiver: PublicKey
): Promise<TransactionInstruction> => {
	const organizationVault = await pda.organizationVault(
		program,
		organization
	);
	const organizationData = await program.account.organization.fetch(
		organization
	);
	const teamTokenMint = organizationData.teamTokenMint;
	const receiverTeamAccount = await pda.ata(receiver, teamTokenMint);

	const ix = await program.methods
		.addTeamMember()
		.accounts({
			organization: organization,
			organizationVault: organizationVault,
			teamTokenMint: teamTokenMint,
			receiver: receiver,
			receiverTeamAccount: receiverTeamAccount,
			systemProgram: SystemProgram.programId,
			tokenProgram: TOKEN_PROGRAM_ID,
			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
			rent: anchor.web3.SYSVAR_RENT_PUBKEY,
		})
		.signers([])
		.instruction();

	return ix;
};
