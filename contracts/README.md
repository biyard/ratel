# Biyard Smart Contracts

This directory contains the Solidity smart contracts for the Biyard platform, including the BiyardToken (ERC20) and DAOTreasury governance system with integrated token exchange.

## Overview

### BiyardToken

An ERC20 token with extended functionality:
- **Minting**: Controlled minting with role-based access
- **Burning**: Users can burn their own tokens
- **Pausable**: Owner can pause/unpause transfers
- **Max Supply**: Optional supply cap to prevent unlimited minting

### DAOTreasury

A decentralized autonomous organization treasury contract with proposal-based governance:
- **Proposal System**: Token holders can create proposals for fund transfers
- **Voting Mechanism**: Token-weighted voting on proposals
- **Quorum Requirements**: Configurable quorum percentage
- **Multi-Asset Support**: Supports both native tokens (ETH/MATIC) and ERC20 tokens
- **Time-Based Voting**: Configurable voting periods
- **Proposal Lifecycle**: Complete proposal state management (Pending, Active, Succeeded, Defeated, Executed, Cancelled)

### DAOTreasuryWithExchange (Enhanced)

The enhanced treasury contract includes all governance features PLUS:
- **Automated Token Exchange**: Built-in DEX for USDT ↔ BiyardToken
- **Dynamic Pricing**: Price calculated from reserves: `Price = USDT Reserve / Circulating BIYARD Supply`
- **Slippage Protection**: Minimum output amount parameter on exchanges
- **Exchange Fees**: Configurable fee percentage (default 0.3%)
- **Reserve Management**: Separate tracking of treasury reserves vs circulating supply
- **Enable/Disable**: Owner can enable/disable exchange functionality

## Project Structure

```
contracts/
├── contracts/
│   ├── BiyardToken.sol                # ERC20 token implementation
│   ├── DAOTreasury.sol                # DAO treasury with governance
│   ├── DAOTreasuryWithExchange.sol    # Enhanced DAO with token exchange
│   └── MockUSDT.sol                   # Mock USDT for testing
├── test/
│   ├── BiyardToken.test.ts            # Token contract tests (20 tests)
│   ├── DAOTreasury.test.ts            # Treasury contract tests (31 tests)
│   └── DAOTreasuryWithExchange.test.ts # Exchange tests (32 tests)
├── scripts/
│   ├── deploy.ts                      # Basic deployment script
│   └── deployWithExchange.ts          # Deployment with exchange setup
├── hardhat.config.ts                  # Hardhat configuration
├── package.json                       # Dependencies
└── README.md                          # This file
```

## Prerequisites

- Node.js 18+
- pnpm 10+

## Installation

From the root of the monorepo:

```bash
pnpm install
```

Or from the contracts directory:

```bash
cd contracts
pnpm install
```

## Development Commands

### Compile Contracts

```bash
pnpm --filter @biyard/contracts compile
```

### Run Tests

```bash
pnpm --filter @biyard/contracts test
```

All tests should pass:
- BiyardToken: 20 tests
- DAOTreasury: 31 tests
- **Total: 51 tests passing**

### Run Local Node

```bash
pnpm --filter @biyard/contracts node
```

### Deploy to Local Network

In one terminal, start the local node:
```bash
pnpm --filter @biyard/contracts node
```

In another terminal, deploy:
```bash
pnpm --filter @biyard/contracts deploy:local
```

## Contract Details

### BiyardToken

**Constructor Parameters:**
- `name`: Token name (e.g., "Biyard Token")
- `symbol`: Token symbol (e.g., "BIYARD")
- `initialSupply`: Initial token supply to mint to deployer
- `maxSupply`: Maximum supply cap (0 for unlimited)

**Key Functions:**
- `mint(address to, uint256 amount)`: Mint new tokens (minter only)
- `burn(uint256 amount)`: Burn your own tokens
- `addMinter(address account)`: Grant minter role (owner only)
- `removeMinter(address account)`: Revoke minter role (owner only)
- `pause()`: Pause token transfers (owner only)
- `unpause()`: Resume token transfers (owner only)

### DAOTreasury

**Constructor Parameters:**
- `_governanceToken`: Address of the ERC20 token used for voting
- `_proposalThreshold`: Minimum tokens needed to create a proposal
- `_votingPeriod`: Voting period in seconds
- `_quorumPercentage`: Percentage of total supply needed for quorum (1-100)

**Key Functions:**

**Treasury Management:**
- `depositTokens(address token, uint256 amount)`: Deposit ERC20 tokens
- `receive()`: Accept native token deposits (ETH/MATIC)
- `getTreasuryBalance(address tokenAddress)`: Check balance

**Proposal Management:**
- `createProposal(string description, address recipient, uint256 amount, address tokenAddress)`: Create new proposal
- `castVote(uint256 proposalId, bool support)`: Vote on a proposal
- `executeProposal(uint256 proposalId)`: Execute a passed proposal
- `cancelProposal(uint256 proposalId)`: Cancel a proposal (proposer or owner only)

**View Functions:**
- `getProposal(uint256 proposalId)`: Get proposal details
- `getProposalState(uint256 proposalId)`: Get proposal state
- `hasVoted(uint256 proposalId, address voter)`: Check if address has voted

**Governance Configuration (Owner Only):**
- `updateProposalThreshold(uint256)`: Update minimum tokens for proposal creation
- `updateVotingPeriod(uint256)`: Update voting period
- `updateQuorumPercentage(uint256)`: Update quorum requirement

### DAOTreasuryWithExchange

**Constructor Parameters:**
- `_governanceToken`: BiyardToken address
- `_usdtToken`: USDT token address
- `_proposalThreshold`: Minimum tokens for proposals
- `_votingPeriod`: Voting period in seconds
- `_quorumPercentage`: Quorum percentage (1-100)
- `_exchangeFeePercentage`: Fee percentage (e.g., 30 = 0.3%)

**Exchange Functions:**
- `getCurrentPrice()`: Get current BIYARD price in USDT
- `getExchangeInfo()`: Get detailed exchange information
- `calculateBiyardToUsdt(uint256 biyardAmount)`: Calculate USDT output for BIYARD input
- `calculateUsdtToBiyard(uint256 usdtAmount)`: Calculate BIYARD output for USDT input
- `exchangeBiyardForUsdt(uint256 biyardAmount, uint256 minUsdtOut)`: Exchange BIYARD for USDT
- `exchangeUsdtForBiyard(uint256 usdtAmount, uint256 minBiyardOut)`: Exchange USDT for BIYARD

**Exchange Configuration (Owner Only):**
- `setExchangeEnabled(bool enabled)`: Enable/disable exchange
- `setExchangeFee(uint256 newFee)`: Update exchange fee

**Price Formula:**
```
Price (USDT per BIYARD) = USDT Reserve / Circulating BIYARD Supply
Circulating Supply = Total BIYARD Supply - Treasury BIYARD Reserve
```

All governance functions from DAOTreasury are also available.

## Deployment Configuration

The default deployment script (`scripts/deploy.ts`) deploys:

1. **BiyardToken**
   - Name: "Biyard Token"
   - Symbol: "BIYARD"
   - Initial Supply: 1,000,000 BIYARD
   - Max Supply: 10,000,000 BIYARD

2. **DAOTreasury**
   - Governance Token: BiyardToken
   - Proposal Threshold: 100 BIYARD
   - Voting Period: 3 days
   - Quorum: 10%

The script also transfers 100,000 BIYARD tokens to the treasury for testing.

### Enhanced Deployment with Exchange

The exchange deployment script (`scripts/deployWithExchange.ts`) deploys:

1. **BiyardToken** (same as above)
2. **MockUSDT** (for testing - use real USDT in production)
   - Symbol: "USDT"
   - Decimals: 6
3. **DAOTreasuryWithExchange**
   - All governance parameters same as above
   - Exchange Fee: 0.3%
   - Initial Reserves: 100,000 BIYARD + 100,000 USDT
   - Initial Price: ~0.111 USDT per BIYARD

Run with: `pnpm --filter @biyard/contracts deploy:exchange`

## Usage Examples

### Token Exchange

```typescript
// Get current exchange information
const info = await treasury.getExchangeInfo();
console.log("Current price:", info.currentPrice);
console.log("USDT reserve:", info.usdtReserve);
console.log("BIYARD reserve:", info.biyardReserve);

// Exchange BIYARD for USDT
const biyardAmount = ethers.parseEther("100"); // 100 BIYARD
const [expectedUsdt, fee] = await treasury.calculateBiyardToUsdt(biyardAmount);

await biyardToken.approve(treasuryAddress, biyardAmount);
await treasury.exchangeBiyardForUsdt(
  biyardAmount,
  expectedUsdt * 95n / 100n  // 5% slippage tolerance
);

// Exchange USDT for BIYARD
const usdtAmount = 1000n * 10n ** 6n; // 1000 USDT
const [expectedBiyard, fee2] = await treasury.calculateUsdtToBiyard(usdtAmount);

await usdtToken.approve(treasuryAddress, usdtAmount);
await treasury.exchangeUsdtForBiyard(
  usdtAmount,
  expectedBiyard * 95n / 100n  // 5% slippage tolerance
);
```

### Creating and Executing a Proposal

```typescript
// 1. User creates a proposal (must have >= 100 BIYARD)
await treasury.createProposal(
  "Fund development team",
  recipientAddress,
  ethers.parseEther("1000"),
  tokenAddress
);

// 2. Token holders vote
await treasury.connect(voter1).castVote(proposalId, true);  // Vote yes
await treasury.connect(voter2).castVote(proposalId, false); // Vote no

// 3. Wait for voting period to end (3 days)

// 4. Execute the proposal
await treasury.executeProposal(proposalId);
```

## Security Features

- **Access Control**: Role-based permissions using OpenZeppelin's Ownable
- **Reentrancy Protection**: SafeERC20 and ReentrancyGuard
- **Pausable Transfers**: Emergency pause mechanism for token
- **Supply Cap**: Optional maximum supply to prevent inflation
- **Voting Integrity**: One vote per address, voting power based on token balance at vote time
- **Proposal Validation**: Checks for sufficient treasury balance before proposal creation

## Testing

The test suite includes comprehensive coverage:

**BiyardToken Tests:**
- Deployment and initialization
- Minting permissions and limits
- Minter role management
- Token burning
- Pausable functionality
- Token transfers

**DAOTreasury Tests (31 tests):**
- Deployment and configuration
- Native and ERC20 token deposits
- Proposal creation and validation
- Voting mechanism
- Proposal execution
- Proposal cancellation
- Governance parameter updates
- View functions

**DAOTreasuryWithExchange Tests (32 tests):**
- Exchange deployment and initialization
- Dynamic price calculation based on reserves
- BIYARD to USDT exchange calculations
- USDT to BIYARD exchange calculations
- Exchange execution with slippage protection
- Exchange fee calculations
- Exchange enable/disable functionality
- Fee configuration updates
- Price impact from exchanges
- Governance integration with exchange
- Edge cases and error handling

**Total: 83 tests passing**

Run tests with:
```bash
pnpm --filter @biyard/contracts test
```

Test output shows all contracts functioning correctly with comprehensive coverage of:
- Token operations
- Governance workflows
- Exchange mechanisms
- Security controls
- Edge cases

## License

MIT

## Integration with Biyard Platform

These contracts are designed to integrate with the Biyard platform's backend APIs:
- The token can be minted through platform APIs when users earn points
- The DAO treasury manages community funds and governance
- Proposals can be created through the platform UI and voted on by token holders
- The backend can listen to contract events to update the database
- **Token Exchange**: Users can swap between BIYARD and USDT directly through the DAO treasury
- **Price Discovery**: Dynamic pricing based on reserve ratios provides automatic market-making
- **Event Listening**: Backend should listen to `TokensExchanged` events to track trades
