import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/C9AZSZiHw9MNTQr5s6xfFMK7yDtvUqRdFaEGEE38sy1r"
        const metadata = {
            name: "Dark rug",
            symbol: "DRG",
            description: "A dark rug that absorbs all light",
            image,  
            attributes: [
                {trait_type: 'power', value: 'dark'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri:  image,
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
        const transactionId = myUri.split('/').pop();
        const devnetGatewayUrl = `https://gateway.irys.xyz/${transactionId}`;
        console.log(`\nViewable Devnet Link: ${devnetGatewayUrl}`);
        console.log("\nNote: It may take 1-5 minutes for the link to become active.");
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
