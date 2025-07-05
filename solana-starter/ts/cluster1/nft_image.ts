import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const imageBuff = await readFile("/home/xcurx/Programming/BlockChain/smart-contract/solana/Q3_25_Builder_SujalKachhawah/solana-starter/ts/cluster1/generug.png")

        //2. Convert image to generic file.
        const genericFile = createGenericFile(new Uint8Array(imageBuff), 'generug.png', { contentType: 'image/png'});
        //3. Upload image

        const [myUri] = await umi.uploader.upload([genericFile]);
        console.log("Your image URI: ", myUri);

        const transactionId = myUri.split('/').pop();
        const devnetGatewayUrl = `https://gateway.irys.xyz/${transactionId}`;
        console.log(`\nViewable Devnet Link: ${devnetGatewayUrl}`);
        console.log("\nNote: It may take 1-5 minutes for the link to become active.");
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();   
