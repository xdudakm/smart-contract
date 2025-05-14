<template>
  <div class="app-container">
    <h1 class="title">🗳️ DeVot</h1>

    <div v-if="!connected" class="button-container">
      <button @click="connect">Connect Wallet</button>
    </div>

    <div v-else class="connected-container">
      <p class="account-info">Connected as: {{ account.meta.name }} ({{ account.address }})</p>

      <h2 class="sub-title">Candidates</h2>
      <ul class="candidate-list">
        <li v-for="c in candidates" :key="c.id" class="candidate-item">
          <span class="candidate-name">{{ c.name }}</span> Votes: {{ c.voteCount }}
          <button @click="vote(c.id)" :disabled="voted" class="vote-button">Vote</button>
        </li>
      </ul>

      <p v-if="voted" class="voted-message">✅ You have already voted.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue';
import {web3Accounts, web3Enable, web3FromSource} from "@polkadot/extension-dapp";
import {ApiPromise, WsProvider} from "@polkadot/api";
import {ContractPromise} from "@polkadot/api-contract";
import metadata from "@/contract/devot_contract.json";

const CONTRACT_ADDRESS = import.meta.env.VITE_APP_CONTRACT_ADDRESS;
const WS_PROVIDER = import.meta.env.VITE_APP_WS_PROVIDER;

const connected = ref(false);
const account = ref<any>(null);
const contract = ref<any>(null);
const candidates = ref<any[]>([]);
const voted = ref(false);

async function connect() {
  // get wallet from polkadot{.js} extension
  await web3Enable('DeVot');
  const allAccounts = await web3Accounts();
  account.value = allAccounts[0];

  //create provider instance with websocket connection
  const wsProvider = new WsProvider(WS_PROVIDER);
  //create api for blockchain
  const api = await ApiPromise.create({provider: wsProvider});
  //create api for blockchain
  contract.value = new ContractPromise(api, metadata as any, CONTRACT_ADDRESS);

  // set signer for web3 actions
  const injector = await web3FromSource(account.value.meta.source);
  api.setSigner(injector.signer);

  connected.value = true;
  await Promise.all([
    fetchCandidates(),
    checkHasVoted()
  ]);
  subscribeToVotingEvents();
}

async function fetchCandidates() {
  const {result, output} = await contract.value.query.getCandidates(
      account.value.address, {
        gasLimit: getDefaultGasLimit()
      }
  );
  if (result.isOk) {
    candidates.value = output.toJSON().ok;
  }
}

async function vote(candidateId: number) {
  // query vote to get required gas for the transaction
  const {gasRequired} = await contract.value.query.vote(account.value.address, {
    gasLimit: getDefaultGasLimit()
  }, candidateId);

  const tx = await contract.value.tx.vote({
    gasLimit: await contract.value.api.registry.createType('WeightV2', {
      refTime: gasRequired.refTime,
      proofSize: gasRequired.proofSize
    })
  }, candidateId);

  // sing the transaction and update list of candidates after block is successfully added
  await tx.signAndSend(account.value.address, (res) => {
    if (res.status.isInBlock || res.status.isFinalized) {
      voted.value = true;
      fetchCandidates();
    }
  });
}

async function checkHasVoted() {
  const {result, output} = await contract.value.query.hasVoted(
      account.value.address, {
        gasLimit: getDefaultGasLimit()
      }, account.value.address);
  if (result.isOk) {
    voted.value = output.toPrimitive().ok;
  }
}

function getDefaultGasLimit() {
  // create WeightV2 type for gas limit
  return contract.value.api.registry.createType('WeightV2', {
    refTime: Number.MAX_SAFE_INTEGER,
    proofSize: Number.MAX_SAFE_INTEGER
  });
}

/**
 * subscribe to events
 * when smart contract is called, update list of candidates with their votes
 */
function subscribeToVotingEvents() {
  contract.value.api.query.system.events((events) => {
    events.forEach((record) => {
      if (record.event.section === 'contracts' && record.event.method === 'ContractEmitted') {
        const decodedRecord = contract.value.abi.decodeEvent(record)
        if (decodedRecord.event.identifier === 'devot_contract::voting::VoteCast') {
          fetchCandidates();
        }
      }
    });
  });
}

</script>

<style>
.app-container {
  font-family: Arial, sans-serif;
  padding: 20px;
  max-width: 800px;
  margin: auto;
}

.title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 20px;
  color: #333;
  text-align: center;
}

.button-container {
  display: flex;
  justify-content: center;
}

button {
  padding: 10px 20px;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

button:hover {
  background-color: #3b36c5;
}

.connected-container {
  margin-top: 30px;
}

.account-info {
  font-size: 1.2rem;
  margin-bottom: 20px;
}

.sub-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 10px;
}

.candidate-list {
  list-style: none;
  padding: 0;
}

.candidate-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #ddd;
}

.candidate-name {
  font-weight: bold;
}

.vote-button {
  padding: 6px 12px;
  background-color: #48bb78;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.vote-button:disabled {
  background-color: #ccc;
}

.vote-button:hover {
  background-color: #38a169;
}

.voted-message {
  font-size: 1.1rem;
  color: green;
  margin-top: 20px;
  font-weight: bold;
}
</style>
