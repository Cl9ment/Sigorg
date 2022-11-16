import * as anchor from "@project-serum/anchor";
import { Sigorg } from "../../types";
import { TransactionInstruction, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { pda } from "../../utils";

export const removeTeamMember = async (
	program: anchor.Program<Sigorg>,
	organization: PublicKey,
	receiver: PublicKey
): Promise<TransactionInstruction> => {
	const organizationData = await program.account.organization.fetch(
		organization
	);
	const teamTokenMint = organizationData.teamTokenMint;
	const receiverTeamAccount = await pda.ata(receiver, teamTokenMint);

	const ix = await program.methods
		.removeTeamMember()
		.accounts({
			organization: organization,
			teamTokenMint: teamTokenMint,
			receiver: receiver,
			receiverTeamAccount: receiverTeamAccount,
			tokenProgram: TOKEN_PROGRAM_ID,
		})
		.signers([])
		.instruction();

	return ix;
};
