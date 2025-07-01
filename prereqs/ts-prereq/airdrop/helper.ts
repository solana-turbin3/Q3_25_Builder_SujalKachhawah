import bs58 from 'bs58';

export const toWallet = (base58Str: string) => {
    const secretKey = bs58.decode(base58Str);
    console.log(secretKey); // Uint8Array of 64 numbers
    return secretKey;
}

export const toBase58 = (secretKey: Uint8Array) => {
    const base58Encoded = bs58.encode(secretKey);
    console.log(base58Encoded); // Phantom-importable format
    return base58Encoded;
}