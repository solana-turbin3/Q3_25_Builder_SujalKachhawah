import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    findMetadataPda
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";

// Define our Mint address
const mint = publicKey("FLeZedjLyRQMYF2sGyKZcx689MZsS9HwrZzkCvTxLphs")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));
console.log(`Using keypair: ${keypair.publicKey}`);

const metadataPda = findMetadataPda(umi, { mint });

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            metadata: metadataPda,
            mint: mint,
            mintAuthority: signer,
            payer: signer,
            updateAuthority: signer,
        }

        let data: DataV2Args = {
            name: "Xcurx",
            symbol: "XRX",
            uri: "https://avatars.githubusercontent.com/u/151933360?v=4",
            sellerFeeBasisPoints: 500, // 5%
            creators: null, // No creators
            collection: null, // No collection
            uses: null, // No uses
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true, // Metadata account is mutable
            collectionDetails: null, // No collection details
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
