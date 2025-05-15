# Smart contract example

This project is basically a dApp for elections - DeVot. On smart contract, candidates and votes are stored. One can vote
using
their wallet.

The project contains 3 packages:

1. [substrate-docekr](substrate-docker) - a docker-compose with substrate contract node for starting local blockchain
   node
2. [devot-contract](devot-contract) - a DeVot ink contract for elections, which contains candidates and votes
3. [devot-web](devot-web) - a DeVot web interface for voting (interacting with the smart contract)

## Run the DeVot dApp

### 1. Run blockchain locally

Start substrate node contract node. It is a "blockchain", which contains support for smart contract. Run following
command in [substrate-docekr](substrate-docker) folder

``` bash
docker compose -f ./substrate-docker/docker-compose.yaml up
```

You can see the chain blocks from [polkadot.js.org](https://polkadot.js.org/apps).

1. Open the website and
2. Select Development Local Node from top left dropdown
3. Click Switch

### 2. Deploy smart contract

The smart contract is in [devot-contract](devot-contract) package. It is written in rust using [ink!](https://use.ink/).
In order to deploy it, we must generate the contract metadata.

1. Install rust from [rust-lang.org](https://www.rust-lang.org/tools/install) (if you don't have it yet)
2. Navigate to [devot-contract](devot-contract) directory

```bash
cd devot-contract
```

3. Install cargo contract with command

```bash
cargo install cargo-contract
```

4. Execute the build

```bash
cargo contract build
```

The build command generated [ink](devot-contract/target/ink)
with [.contract](devot-contract/target/ink/devot_contract.contract)
and [.json](devot-contract/target/ink/devot_contract.json). This file contains all the information about the smart
contract.

For deploying and testing ink! smart contract, we can use [use.ink](https://ui.use.ink/)

1. Open https://ui.use.ink
2. From top left dropdown, select Local Node
3. Click Add new contract
4. Upload generated [.contract](devot-contract/target/ink/devot_contract.contract) file.
5. Click Next
6. Enter names of candidates
7. Click Next and Upload and Instantiate

Now, you can interact with the deployed contract from the web interface by calling the selected method.

### 3. Crate wallet

1. Install [polkadot extension](https://polkadot.js.org/extension/)
2. Create account from the extension
3. As we are running development node, there are test users with some units. Transfer 10 units from test user to your
   account from [accounts](https://polkadot.js.org/apps/#/accounts).

### 4. Run dApp

1. Install [Node.js](https://nodejs.org/en/download) if you don't have it yet
2. Navigate to [devot-web](devot-web) directory

```bash
cd devot-web
```

3. Build the project

```bash
npm install
cp .env.example .env
```

4. Copy .json contract from [.json](devot-contract/target/ink) and paste it in [contract](devot-web/src/contract)
   directory in web package.

```bash
cp ../devot-contract/target/ink/devot_contract.json ./src/contract/
```

5. Copy address of the deployed smart contract from [use.ink](https://ui.use.ink/) and set it as value of
   VITE_APP_CONTRACT_ADDRESS in the [.env](devot-web/.env) file.

5. Run the app

```bash
npm run dev -- --host
```

6. Open the app in the browser http://localhost:5174/
7. Connect your wallet and vote.
8. Connect to your classmate via exposed ip. Vote on their smart contract and see what happens on the website.
9. Inspect code of both smart contract and web app to see, what has happened under the hood.

## Customize the smart contract

Now your task is to change this smart contract to the one for selling e-books.

### Change smart contract

1. Change storage to BookStore with following values:

- books: Mapping<u32, Book>,
- num_books: u32

Book will have owner(AccountId), title, author name, content hash, price, and sold flag.

2. Implement the BookStore

In constructor, initiate list of books you want to put on market.

3. Create message functions

- add_book(title, author, content_hash, price) -> Add book to the storage
- buy(book_id) -> Change the book ownership in the contract storage and set it to sold.
- sell(book_id) -> Change sold flag to false of existing book that is owned by the caller.
- get_content(book_id) -> Only the owner of the book can get the content
- owned_books() -> List of books that are owned by the caller.
- list_books() -> List of books that haven't been sold yet and are not owned by the caller.

4. Deploy the smart contract
5. Update frontend to interact with the newly deployed smart contract