# Ratel Smart Contracts

This directory contains the Solidity smart contracts for the Ratel platform, featuring a modular DAO system for decentralized space governance.

## Overview

### SpaceDAO

A lightweight, upgradeable DAO core contract that manages admin permissions and delegates functionality to extensions:
- **Admin Management**: Multi-admin governance (minimum 3 admins required)
- **Extension System**: Whitelist-based extension mechanism for modular functionality
- **Emergency Controls**: DAO activation/deactivation switch
- **Call Delegation**: Secure execution of calls from authorized extensions
- **Upgradeable**: Uses OpenZeppelin's Initializable pattern with Clones

### RewardExtension

A governance extension for managing batch reward distributions with multi-signature approval:
- **Proposal System**: Admins can create batch transfer proposals
- **Multi-Signature Approval**: Requires 2 out of 3 admin approvals for execution
- **Batch Transfers**: Support for multiple recipients in a single proposal
- **Multi-Asset Support**: Supports both native tokens (ETH/KAIA) and ERC20 tokens
- **Auto-Execution**: Automatically executes when quorum is reached
- **Proposal Tracking**: Complete proposal state management and history

### SpaceFactory

A factory contract for deploying DAO instances using the Clone pattern:
- **Gas-Efficient Deployment**: Uses EIP-1167 minimal proxy clones
- **Atomic Deployment**: Creates and links DAO + Extension in one transaction
- **Deployment Registry**: Tracks all deployed DAOs
- **Immutable Logic**: Implementation contracts are immutable and reusable

## Project Structure

```
contracts/
├── contracts/
│   ├── space_dao/
│   │   ├── SpaceDao.sol                    # Core DAO contract
│   │   ├── SpaceDaoRewardExtension.sol     # Reward distribution extension
│   │   └── SpaceDaoFactory.sol             # Factory for deploying DAOs
│   └── mocks/
│       └── MockToken.sol                   # Mock ERC20 for testing
├── tests/
│   └── SpaceDao.test.ts                    # Comprehensive test suite (7 tests)
├── hardhat.config.ts                       # Hardhat configuration
├── package.json                            # Dependencies
└── README.md                               # This file
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
pnpm --filter @ratel/contracts compile
```

### Run Tests

```bash
pnpm --filter @ratel/contracts test
```

All tests should pass:
- SpaceDAO System: 7 tests
  - Deployment & Linking: 2 tests
  - Governance (Propose & Approve): 3 tests
  - Execution (Funds): 2 tests
- **Total: 7 passing**

### Run Local Node

```bash
pnpm --filter @ratel/contracts node
```

### Deploy to Local Network

In one terminal, start the local node:
```bash
pnpm --filter @ratel/contracts node
```

In another terminal, deploy:
```bash
pnpm --filter @ratel/contracts deploy:local
```

## Contract Details

### SpaceDAO

**Initialization Parameters:**
- `_admins`: Array of admin addresses (minimum 3 required)
- `_initialExtension`: Address of the initial extension to register

**Key Functions:**

**Extension Management:**
- `executeCall(address target, uint256 value, bytes calldata data)`: Execute calls from authorized extensions
- `checkAdmin(address user)`: Check if an address has admin privileges

**State Variables:**
- `isDaoActive`: Emergency pause switch
- `admins`: Array of admin addresses
- `isAdmin`: Mapping to check admin status
- `isExtension`: Whitelist of authorized extensions
- `rewardExtension`: Direct reference to the main reward extension

**Events:**
- `Initialized(address[] admins, address extension)`: Emitted when DAO is initialized
- `ExtensionCall(address indexed extension, address indexed target, uint256 value, bytes data)`: Emitted when extension executes a call
- `Received(address indexed sender, uint256 amount)`: Emitted when DAO receives native tokens

### RewardExtension

**Initialization Parameters:**
- `_dao`: Address of the SpaceDAO contract

**Key Functions:**

**Proposal Management:**
- `proposeBatch(address token, TransferPair[] calldata pairs)`: Create a new batch transfer proposal
  - `token`: Token address (address(0) for native tokens)
  - `pairs`: Array of recipient-amount pairs (max 100)
- `approveAndExecute(uint256 id)`: Approve a proposal and execute if quorum is reached
- `getProposalInfo(uint256 id)`: Get proposal details

**Structures:**
- `TransferPair`: `{ address recipient, uint256 amount }`
- `Proposal`: Contains token, pairs, approval count, execution status, and approver mapping

**Constants:**
- `REQUIRED_APPROVALS`: 2 (out of 3+ admins)

**Events:**
- `ProposalCreated(uint256 indexed id, address indexed proposer, uint256 count)`: New proposal created
- `Approved(uint256 indexed id, address indexed approver)`: Admin approved a proposal
- `BatchExecuted(uint256 indexed id)`: Proposal successfully executed

### SpaceFactory

**Constructor Parameters:**
- `_daoImpl`: Address of SpaceDAO implementation contract
- `_extImpl`: Address of RewardExtension implementation contract

**Key Functions:**
- `createSpace(address[] calldata admins)`: Deploy a new DAO instance with its extension
  - Returns the address of the deployed DAO
- `getDeployedDAOs()`: Get list of all deployed DAO addresses

**Events:**
- `SpaceCreated(address indexed dao, address indexed rewardExtension)`: Emitted when a new space is created

## Deployment

The SpaceDAO system uses a factory pattern for efficient deployment:

### Deployment Steps

1. **Deploy Implementation Contracts**
   ```typescript
   const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
   const spaceDaoLogic = await SpaceDAO.deploy();

   const RewardExtension = await ethers.getContractFactory("RewardExtension");
   const rewardExtLogic = await RewardExtension.deploy();
   ```

2. **Deploy Factory**
   ```typescript
   const SpaceFactory = await ethers.getContractFactory("SpaceFactory");
   const factory = await SpaceFactory.deploy(
     await spaceDaoLogic.getAddress(),
     await rewardExtLogic.getAddress()
   );
   ```

3. **Create DAO Instances**
   ```typescript
   const admins = [admin1.address, admin2.address, admin3.address];
   const tx = await factory.createSpace(admins);
   ```

This approach uses EIP-1167 minimal proxy clones for gas-efficient deployments.

## Usage Examples

### Creating a Batch Reward Proposal

```typescript
// 1. Admin creates a proposal for batch rewards
const pairs = [
  { recipient: user1.address, amount: ethers.parseEther("100") },
  { recipient: user2.address, amount: ethers.parseEther("200") },
  { recipient: user3.address, amount: ethers.parseEther("150") }
];

// For native tokens (ETH/KAIA)
await rewardExtension.connect(admin1).proposeBatch(
  ethers.ZeroAddress,  // address(0) for native tokens
  pairs
);

// For ERC20 tokens
await rewardExtension.connect(admin1).proposeBatch(
  tokenAddress,  // ERC20 token address
  pairs
);
```

### Approving and Executing a Proposal

```typescript
// 2. Second admin approves (automatically executes when quorum is reached)
await rewardExtension.connect(admin2).approveAndExecute(proposalId);

// The proposal executes automatically when 2 approvals are reached
// All recipients receive their tokens in a single transaction
```

### Funding the DAO

```typescript
// Fund DAO with native tokens
await admin1.sendTransaction({
  to: daoAddress,
  value: ethers.parseEther("10")
});

// Fund DAO with ERC20 tokens
await token.transfer(daoAddress, ethers.parseEther("1000"));
```

### Checking Proposal Status

```typescript
// Get proposal information
const info = await rewardExtension.getProposalInfo(proposalId);
console.log("Token:", info.token);
console.log("Recipients count:", info.count);
console.log("Approvals:", info.approvals);
console.log("Executed:", info.executed);
```

## Security Features

- **Multi-Admin Governance**: Requires minimum 3 admins for redundancy
- **Multi-Signature Approval**: 2-of-3 approval quorum prevents single-point-of-failure
- **Reentrancy Protection**: Uses OpenZeppelin's ReentrancyGuard
- **Extension Whitelist**: Only authorized extensions can execute DAO calls
- **Emergency Pause**: DAO can be deactivated in case of emergency
- **Upgradeable Pattern**: Uses Initializable with constructor disabled
- **Clone Pattern**: EIP-1167 minimal proxies for gas efficiency and security
- **Admin-Only Actions**: Only registered admins can create and approve proposals
- **Batch Limits**: Maximum 100 recipients per proposal to prevent gas issues

## Testing

The test suite includes comprehensive coverage:

**Deployment & Linking (2 tests):**
- DAO and Extension deployment and initialization
- Admin registration and verification
- Extension-DAO linking

**Governance (3 tests):**
- Proposal creation by admins
- Access control (non-admin rejection)
- Auto-execution when quorum is reached
- Approval tracking

**Execution (2 tests):**
- Native token (ETH/KAIA) transfers
- ERC20 token transfers
- Batch transfers to multiple recipients
- Balance verification

**Total: 7 tests passing**

Run tests with:
```bash
pnpm --filter @ratel/contracts test
```

Test output shows all contracts functioning correctly with comprehensive coverage of:
- Factory pattern deployment
- Multi-signature governance
- Batch reward distribution
- Access control
- Security features

## License

MIT

## Integration with Ratel Platform

These contracts are designed to integrate with the Ratel platform's backend APIs:

### Event Listening
The backend should listen to the following events for database synchronization:

**SpaceFactory Events:**
- `SpaceCreated(address dao, address rewardExtension)`: Track newly created spaces

**RewardExtension Events:**
- `ProposalCreated(uint256 id, address proposer, uint256 count)`: Record new proposals
- `Approved(uint256 id, address approver)`: Track approval progress
- `BatchExecuted(uint256 id)`: Update proposal execution status

**SpaceDAO Events:**
- `ExtensionCall(address extension, address target, uint256 value, bytes data)`: Monitor extension activities
- `Received(address sender, uint256 amount)`: Track treasury funding

### Integration Points

1. **Space Creation**: Backend API can call the factory to create new DAOs for spaces
2. **Reward Distribution**: Platform can trigger batch reward proposals through admin accounts
3. **Admin Management**: Initial admins are set during space creation from platform data
4. **Treasury Monitoring**: Track DAO balances for both native tokens and ERC20s
5. **Proposal Tracking**: Maintain proposal history and status in the backend database

### Gas Optimization

The system uses EIP-1167 minimal proxy clones, making each new DAO deployment cost approximately **456,070 gas** - significantly cheaper than deploying full contracts.
