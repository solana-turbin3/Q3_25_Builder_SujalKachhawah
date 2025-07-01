import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./wallet.json"
import { toWallet } from "./helper";
import { SystemProgram } from "@solana/web3.js";
const SYSTEM_PROGRAM_ID = SystemProgram.programId;
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

const keypair = Keypair.fromSecretKey(toWallet(wallet.privateKey));
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"}
);

const program : Program<Turbin3Prereq> = new Program(IDL, provider);

const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintTs = Keypair.generate();

const authoritySeeds = [
  Buffer.from("collection"), // const seed
  mintCollection.toBuffer(), // account seed
];

const [authorityPda] = PublicKey.findProgramAddressSync(
  authoritySeeds,
  program.programId
);

// Execute the initialize transaction
// (async () => {
//     try {
//     const txhash = await program.methods
//     .initialize("xcurx")
//     .accountsPartial({
//         user: keypair.publicKey,
//         account: account_key,
//         system_program: SYSTEM_PROGRAM_ID,
//     })
//     .signers([keypair])
//     .rpc();
//     console.log(`Success! Check out your TX here:
//     https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
// } catch (e) {
//     console.error(`Oops, something went wrong: ${e}`);
// }
// })();

(async () => {
try {
const txhash = (await program.methods as any)
    ["submitTs"]()
    .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: authorityPda,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
    })
    .signers([keypair, mintTs])
    .rpc();
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
} catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
}
})()