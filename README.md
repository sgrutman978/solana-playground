```solana-keygen recover --force
solana airdrop 2
solana-keygen new --force  
solana balance
solana address
solana program deploy /Users/stevengrutman/Documents/hello_world/target/deploy/hello_world.so
solana config set --url http://127.0.0.1:8899
solana config set -u devnet|testnet|mainnet-beta
cargo build-bpf

client:
    npm run start```