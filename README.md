# Technical challenge

In this test you are required to build a simple OTC (over-the-counter) contract. Scammers typically use OTC deals to try and trick sellers with fake tokens to gain whatever they are trying to buy without actually paying. A contract can be used to stop these scammers.
For this contract there will be two sides, a buyer and a seller. The buyer or seller should be able to create a deal, but neither should have admin rights to change the deal after creation.

## The contract rules

1. Anyone can create an OTC deal
2. The deal must specify the amount and denom of the token A being used to purchase and the amount and denom of token B being sold
3. You only need to support native, IBC and TokenFactory tokens, no need to support CW20 tokens
4. When both sides have deposited their tokens, the other party should be allowed to withdraw the tokens they are owed
5. When both sides have deposited their tokens, they should not be allowed to withdraw their own tokens anymore
6. The deal must have a configurable expiry date, after the expiry date no tokens must be accepted and either party can withdraw their tokens unless (4) or (5) above is reached before expiry
7. You are responsible for creating the query and execute API for the contract
8. Must be built using CosmWasm and Rust
9. You are allowed to use other crates to ease your work

## You do not have to:

1. Build a UI for it
2. Support CW20 tokens
3. Deploy it on a testnet

## Improvements

(2) The create_deal function also has an optional recipient.

(4) When both sides have deposited their tokens, the contract sends funds to both parties at the same time (no need to manually withdraw).

(6) An optional timeout is required on deal creation (if None set, the default timeout is used -> set on instantiation).

(\*) Update deal status from Open to Expired when time's up.

## Assumptions

(1) An OTC deal is a simple swap, regardless of the direction. Meaning, buy A with B is the same as selling B for A. A deal cannot be created without sending funds.

(5) The creator of the deal can withdraw their funds at any time -> no rug possible as the contract sends (denom_out / amount_out) tokens to receiver as soon as they deposit
