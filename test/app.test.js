const fs = require('fs');
const BN = require('bn.js');
const nearAPI = require('near-api-js');
const testUtils = require('./test-utils');
const getConfig = require('../src/config');

const { 
	Contract, KeyPair, Account,
	utils: { format: { parseNearAmount }},
	transactions: { deployContract, functionCall },
} = nearAPI;
const { 
	connection, initContract, getAccount, getAccountBalance,
	contract, contractAccount, contractName, contractMethods, createAccessKeyAccount,
	createOrInitAccount,
	getContract,
} = testUtils;
const { 
	networkId, GAS, GUESTS_ACCOUNT_SECRET
} = getConfig();

jasmine.DEFAULT_TIMEOUT_INTERVAL = 60000;

const now = Date.now();
const tokenIds = ['A-' + now, 'B-' + now, 'C-' + now].map((type, i) => `${type}:${i}`);

const metadata = {
	media: 'https://media1.tenor.com/images/4c1d96a989150e7019bfbabbebd2ff36/tenor.gif?itemid=20269144',
	issued_at: now.toString()
};
const metadata2 = {
	media: 'https://media1.tenor.com/images/818161c07948bac34aa7c5f5712ec3d7/tenor.gif?itemid=15065455',
	issued_at: now.toString()
};

const contractId = contractAccount.accountId;

describe('deploy contract ' + contractName, () => {

	let alice, aliceId, bob, bobId;

	/// most of the following code in beforeAll can be used for deploying and initializing contracts
	/// skip tests if you want to deploy to production or testnet without any NFTs
	beforeAll(async () => {
	    await initContract();

		/// some users
		aliceId = 'alice-' + now + '.' + contractId;
		alice = await getAccount(aliceId);
		console.log('\n\n Alice accountId:', aliceId, '\n\n');

		bobId = 'bob-' + now + '.' + contractId;
		bob = await getAccount(bobId);
		console.log('\n\n Bob accountId:', bobId, '\n\n');

	});

	test('NFT enumerable tests (no tokens)', async () => {
		const nft_supply_for_owner = await bob.viewFunction(contractName, 'nft_supply_for_owner', { account_id: bobId });
		console.log('\n\n nft_supply_for_owner', nft_supply_for_owner, '\n\n');
		expect(nft_supply_for_owner).toEqual('0');
		// messing around with index and limit
		const bobTokens = await bob.viewFunction(contractName, 'nft_tokens_for_owner', {
			account_id: bobId, from_index: '1001', limit: '1000'
		});
		console.log('\n\n bobTokens', bobTokens, '\n\n');
		expect(bobTokens.length).toEqual(0);
	});

	test('contractAccount mints nft', async () => {
		const token_id = tokenIds[0];
		
		await contractAccount.functionCall({
			contractId: contractId,
			methodName: 'nft_mint',
			args: {
				token_id,
				metadata,
				perpetual_royalties: {
					'a1.testnet': 500,
					'a2.testnet': 250,
					'a3.testnet': 250,
					'a4.testnet': 250,
					'a5.testnet': 250,
				},
			},
			gas: GAS,
			attachedDeposit: parseNearAmount('1')
		});

		const token = await contractAccount.viewFunction(contractId, 'nft_token', {
			token_id
		});
		
		expect(token.owner_id).toEqual(contractId);
	});

});