# Astroport technical interview assignment

In this test you are required to build a simple OTC (over-the-counter) contract. Scammers typically use OTC deals to try and trick sellers with fake tokens to gain whatever they are trying to buy without actually paying. A contract can be used to stop these scammers. For this contract there will be two sides, a buyer and a seller. The buyer or seller should be able to create a deal, but neither should have admin rights to change the deal after creation.

The contract rules
Anyone can create an OTC deal
The deal must specify the amount and denom of the token A being used to purchase and the amount and denom of token B being sold
You only need to support native, IBC and TokenFactory tokens, no need to support CW20 tokens
When both sides have deposited their tokens, the other party should be allowed to withdraw the tokens they are owed
When both sides have deposited their tokens, they should not be allowed to withdraw their own tokens anymore
The deal must have a configurable expiry date, after the expiry date no tokens must be accepted and either party can withdraw their tokens unless (4) or (5) above is reached before expiry
You are responsible for creating the query and execute API for the contract
Must be built using CosmWasm and Rust
You are allowed to use other crates to ease your work
You do not have to:
Build a UI for it
Support CW20 tokens
Deploy it on a testnet
Other
This must be your own work. If an OTC contract already exists, you should not use it but rather come up with your own version. You are not allowed to use ChatGPT or similar AI to write the code for you. We are interested in the way you think and build.

While this test doesn't have a specific timeframe to be completed in, we don't want you to spend a week on it. Ideally work like you usually would.

** Please create a private GitHub repo and commit your work as you would normally. Once you are done with the assignment, let us know and we'll share our GitHub usernames to get access to the repo. Please do not share this assignment. **

Chat soon!
