# Smart contract example

This project is a simple example of dApp for decentralized voting - DeVot. On smart contract, candidates and votes are
stored. One can vote using their wallet.

It is used as blockchain and smart contract demonstration for AASS lecture on FIIT STUBA.

The repository consists of 3 packages:

1. [substrate-docker](substrate-docker) - a docker-compose with substrate contract node for starting local blockchain
   node
2. [devot-contract](devot-contract) - a DeVot ink contract for elections, which contains candidates and votes
3. [devot-web](devot-web) - a DeVot web interface for voting (interacting with the smart contract)

## Requirements (To speed up your work at lecture)

1. This repository cloned locally on your machine
2. Installed docker on your machine ([instructions](https://docs.docker.com/engine/install/))
3. Downloaded substrate image (inside [substrate-docker](substrate-docker) folder, run `docker compose pull`)
4. Installed rust compiler on your
   machine ([instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html))
5. Installed cargo-contract (inside [devot-contract](devot-contract) folder, run `cargo install cargo-contract`)
6. Build !ink smart contract (inside [devot-contract](devot-contract) folder, run `cargo contract build`)

## Run the DeVot dApp

### 1. Run polkadot blockchain locally

Start substrate node contract node. It is a "blockchain", which contains support for smart contract. Run following
command in root folder

``` bash
docker compose -f ./substrate-docker/docker-compose.yaml up
```

You can see the chain blocks
from [polkadot.js.org](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).

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

For deploying and testing ink! smart contract, we will
use [polkadot.js](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts/)

1. Open [https://polkadot.js/apps/contracts](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts/)
2. Click upload & deploy code
3. Upload generated [.contract](devot-contract/target/ink/devot_contract.contract) file.
4. Click Next
5. Enter names of candidates
6. Click Deploy and Sign and Submit

Now, you can interact with the deployed contract from the web interface by calling messages available from the dropdown
menu.

### 3. Crate wallet

1. Install [polkadot extension](https://polkadot.js.org/extension/)
2. Create account from the extension
3. In substrate-contract-node chain, each transaction requires som fees to prevent DoS attack or spam (it is possible to
   turn it off by modifying substrate code). As we are running development node, there are test users with some units.
   Transfer 10 units from test user to your account
   from [accounts](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/accounts).

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

5. Copy address of the deployed smart contract
   from [contracts](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts) (click on the contract
   name and you will see its address) and set it as value of VITE_APP_CONTRACT_ADDRESS in the [.env](devot-web/.env)
   file.

6. Run the app

```bash
npm run dev -- --host
```

7. Open the app in the browser http://localhost:5174/
8. Connect your wallet and vote (you mast have it enabled via the extension).
9. Connect to your classmate via exposed ip. Vote on their smart contract and see what happens on the website.
10. Inspect code of both smart contract and web app to see, what has happened under the hood.

If unexpected behavior happens, open web browser console and debug.

## Customize the smart contract

Now your task is to change this smart contract to the one for selling e-books. We will just change the existing smart
contract and frontend. The smart contract is just for demonstration, without using any balance when selling books.

If your contract is not working, you can check against [dBook](https://github.com/xdudakm/dBook) repository, where is
the contract as well as frontend for it.

### 1. Change smart contract

1. Change storage to BookStore with following values:

- books: Mapping<u32, Book>,
- num_books: u32

Book will have id, owner(AccountId), title, author_name, content_hash, price, and for_sale flag.

2. Implement the BookStore

In constructor, initiate list of books you want to put on market.

3. Create message functions

- add_book(title, author, content_hash, price) -> Add book to the storage
- buy(book_id) -> Change the book ownership in the contract storage and set for_sale to false.
- sell(book_id) -> Change for_sale flag to true of existing book that is owned by the caller.
- get_content(book_id) -> Only the owner of the book can get the content
- owned_books() -> List of books that are owned by the caller.
- books_for_sale() -> List of books that are for sale and are not owned by the caller.

### 2. Deploy smart contract

1. Build the smart contract with command

```bash
cargo contract build
```

2. Open [https://polkadot.js/apps/contracts](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts/)
3. Click upload .contract file
4. Set contract name to dBook
5. Add initial books
6. Upload the contract

### 3. Update frontend

1. Copy generated .json contract and paste it in contract directory of web app.
2. Copy address of deployed contract to variable VITE_APP_CONTRACT_ADDRESS in .env
3. Use contract to query and transfer data:

- contract.value.query.ownedBooks(address, {gasLimit})
- contract.value.query.booksForSale(address, {gasLimit})
- contract.value.query.getContent(address, {gasLimit}, book_id)
- contract.value.tx.addBook({gasLimit}, newBook.title, newBook.author, newBook.content, newBook.price);
- contract.value.tx.buy({gasLimit}, id);
- contract.value.tx.buy({gasLimit}, id);
