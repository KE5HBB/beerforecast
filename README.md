# Night Protocol
A simple blockchain written from scratch in the rust programming language. This is a very simple protocol for me to reseach about how the functioning of a blockchain system works. Hoping to make this reachable to the people who are beginners in blockchain and web3 learn about what goes on under the hood of a chain. Follow the [detailed documentatation](https://night-protocol.netlify.app/explore) to know more.

> **Note**
> I'm working on the architecture of user accounts using asymmetric encryption.

## Stack
- Rust and rocket.rs as chain and backend
- Nextjs and mantine for client
- MDX for documentation

## How to start the project
- Make sure to have rust and nodejs installed.
- Clone the repo
- `cd` into the `night_protocol` folder and run `cargo run`
- open another terminal instance and  `cd` into the `client` folder and run `yarn && yarn run dev`
- Alternatively, you can only run the rust server and visit https://night-protocol.netlify.app to set it up.

## How it works

It has 2 chains for now, the transaction chain and the master chain. When there are more than 100 blocks