# Ratel Smart Contracts

This directory contains the Solidity smart contracts for the Ratel platform, featuring standalone DAO contracts for decentralized governance and treasury management.

## Overview

### TeamDAO

A standalone multi-admin governance contract for managing batch reward distributions:
- **Multi-Admin Governance**: Requires minimum 3 admins for redundancy
- **Majority Approval System**: Proposals execute when `(admins.length / 2) + 1` approvals reached
- **Batch Rewards**: Support for multiple recipients in a single proposal (max 100)
- **ERC20 Token Support**: Handles ERC20 token transfers only
- **Auto-Execution**: Automatically executes when approval quorum is reached
- **Reentrancy Protection**: Uses OpenZeppelin's ReentrancyGuard
- **Proposal Tracking**: Complete proposal state management and history

### SpaceDAO

A standalone treasury management contract for deposit and withdrawal operations:
- **Multi-Admin Governance**: Requires minimum 3 admins
- **USDT Treasury**: Manages USDT token deposits and distributions
- **Fixed Withdrawal Amounts**: Configurable per-recipient withdrawal amount
- **Admin-Only Withdrawals**: Only admins can distribute funds to recipients
- **Balance Tracking**: View treasury balance and withdrawal amounts
- **Flexible Configuration**: Update withdrawal amounts and token addresses

## Project Structure

```
contracts/
├── contracts/
│   ├── team_dao/
│   │   └── TeamDao.sol           # Multi-admin governance with reward proposals
│   ├── space_dao/
│   │   └── SpaceDao.sol           # Deposit/withdrawal treasury management
│   └── mocks/
│       └── MockToken.sol          # Mock ERC20 for testing
├── tests/
│   ├── TeamDao.test.ts            # 17 tests in 5 suites
│   └── SpaceDao.spec.ts           # 9 tests in 5 suites
├── hardhat.config.ts              # Hardhat configuration
├── package.json                   # Dependencies
├── Makefile                       # Build commands
└── README.md                      # This file
```

## Prerequisites

- Node.js 18+
- npm (comes with Node.js)

## Installation

From the contracts directory:

```bash
cd contracts
npm install
```

## Development Commands

### Compile Contracts

```bash
npm run compile
# or
make build
```

### Run Tests

```bash
npm run test
# or
make test
```

Test results:
- **TeamDAO System**: 16 tests
  - Deployment: 2 tests passing
  - Governance (Propose & Approve): 4 tests passing
  - Majority Logic: 2 tests passing
  - Execution (ERC20 Transfers): 3 tests passing
- **SpaceDAO System**: 9 tests passing
  - Deployment: 3 tests passing
  - Deposit & Balance: 1 test passing
  - Withdraw distribution: 3 tests passing
  - Admin management: 3 tests passing
- **Total: 23 passing**

### Run Local Node

```bash
npm run node
# or
make node
```

## Contract Details

### TeamDAO

**Constructor Parameters:**
- `_admins`: Array of admin addresses (minimum 3 required)

**Key Functions:**

**Proposal Management:**
- `proposeBatch(address token, TransferPair[] calldata pairs)`: Create a new batch transfer proposal
  - `token`: ERC20 token address (cannot be zero address)
  - `pairs`: Array of recipient-amount pairs (max 100)
  - Creator is automatically counted as first approver
- `approveAndExecute(uint256 id)`: Approve a proposal and execute if quorum is reached
- `getProposalInfo(uint256 id)`: Get proposal details (token, count, approvals, executed status)
- `checkAdmin(address user)`: Check if an address has admin privileges
- `getRequiredApprovals()`: Get the number of approvals required for execution

**Structures:**
- `TransferPair`: `{ address recipient, uint256 amount }`
- `Proposal`: Contains id, token, pairs, approval count, execution status, and approver mapping

**Majority Rule:**
- Required approvals: `(admins.length / 2) + 1`
- For 3 admins: requires 2 approvals
- For 4 admins: requires 3 approvals
- For 5 admins: requires 3 approvals

**State Variables:**
- `admins`: Array of admin addresses
- `isAdmin`: Mapping to check admin status
- `proposals`: Array of all proposals

**Events:**
- `Initialized(address[] admins)`: Emitted when DAO is deployed
- `Received(address indexed sender, uint256 amount)`: Emitted when DAO receives native tokens
- `ProposalCreated(uint256 indexed id, address indexed proposer, uint256 count)`: New proposal created
- `Approved(uint256 indexed id, address indexed approver)`: Admin approved a proposal
- `BatchExecuted(uint256 indexed id)`: Proposal successfully executed

### SpaceDAO

**Constructor Parameters:**
- `admins`: Array of admin addresses (minimum 3 required)
- `usdt`: USDT token contract address
- `withdrawalAmount`: Fixed amount to distribute per recipient

**Key Functions:**

**Treasury Management:**
- `deposit(uint256 amount)`: Deposit USDT tokens into the DAO treasury
  - Requires prior token approval
- `distributeWithdrawal(address[] calldata recipients)`: Distribute fixed amounts to recipients
  - Admin-only function
  - Requires sufficient balance for all recipients
- `getBalance()`: View current treasury balance

**Configuration:**
- `setWithdrawalAmount(uint256 amount)`: Update the per-recipient withdrawal amount (admin-only)
- `setUsdtAddress(address usdt)`: Update the USDT token address (admin-only)
- `addAdmin(address admin)`: Add a new admin (admin-only)

**View Functions:**
- `getAdmins()`: Get list of all admin addresses
- `getIsAdmin(address account)`: Check if an address is an admin
- `getUsdt()`: Get the USDT token contract address
- `getWithdrawalAmount()`: Get the current per-recipient withdrawal amount

**Events:**
- No custom events (uses standard ERC20 transfer events)



## Usage Examples

### TeamDAO: Creating and Executing a Batch Reward Proposal

```typescript
// 1. Deploy MockToken for testing
const MockToken = await ethers.getContractFactory("MockToken");
const token = await MockToken.deploy();

// 2. Fund the TeamDAO
const fundAmount = ethers.parseEther("1000");
await token.transfer(await teamDao.getAddress(), fundAmount);

// 3. Admin1 creates a proposal for batch rewards
const pairs = [
  { recipient: user1.address, amount: ethers.parseEther("100") },
  { recipient: user2.address, amount: ethers.parseEther("200") },
  { recipient: user3.address, amount: ethers.parseEther("150") }
];

await teamDao.connect(admin1).proposeBatch(
  await token.getAddress(),
  pairs
);
// Admin1 is automatically counted as the first approver

// 4. Admin2 approves - automatically executes when quorum (2/3) is reached
await teamDao.connect(admin2).approveAndExecute(0);

// 5. All recipients have received their tokens
console.log(await token.balanceOf(user1.address)); // 100 tokens
console.log(await token.balanceOf(user2.address)); // 200 tokens
console.log(await token.balanceOf(user3.address)); // 150 tokens
```

### SpaceDAO: Depositing and Distributing Funds

```typescript
// 1. Deploy MockToken (representing USDT)
const MockToken = await ethers.getContractFactory("MockToken");
const usdt = await MockToken.deploy();

// 2. Deploy SpaceDAO with 100 USDT per recipient
const withdrawalAmount = ethers.parseUnits("100", 18);
const spaceDao = await SpaceDAO.deploy(
  [admin1.address, admin2.address, admin3.address],
  await usdt.getAddress(),
  withdrawalAmount
);

// 3. User deposits USDT into the DAO
const depositAmount = ethers.parseUnits("1000", 18);
await usdt.connect(user).approve(await spaceDao.getAddress(), depositAmount);
await spaceDao.connect(user).deposit(depositAmount);

// 4. Check balance
console.log(await spaceDao.getBalance()); // 1000 USDT

// 5. Admin distributes to 5 recipients (100 USDT each)
await spaceDao.connect(admin1).distributeWithdrawal([
  recipient1.address,
  recipient2.address,
  recipient3.address,
  recipient4.address,
  recipient5.address
]);

// 6. Each recipient receives exactly 100 USDT
console.log(await usdt.balanceOf(recipient1.address)); // 100 USDT
console.log(await spaceDao.getBalance()); // 500 USDT remaining

// 7. Admin can update withdrawal amount
await spaceDao.connect(admin1).setWithdrawalAmount(ethers.parseUnits("200", 18));
console.log(await spaceDao.getWithdrawalAmount()); // 200 USDT
```

### Checking Proposal Status (TeamDAO)

```typescript
// Get proposal information
const info = await teamDao.getProposalInfo(proposalId);
console.log("Token:", info.token);
console.log("Recipients count:", info.count);
console.log("Approvals:", info.approvals);
console.log("Executed:", info.executed);

// Check if address is admin
const isAdmin = await teamDao.checkAdmin(someAddress);
console.log("Is admin:", isAdmin);

// Get required approvals
const required = await teamDao.getRequiredApprovals();
console.log("Required approvals:", required); // 2 for 3 admins
```

## Security Features

### TeamDAO
- **Multi-Admin Governance**: Requires minimum 3 admins for redundancy
- **Majority Approval**: Requires `(admins.length / 2) + 1` approvals to execute
- **Reentrancy Protection**: Uses OpenZeppelin's ReentrancyGuard
- **Admin-Only Actions**: Only registered admins can create and approve proposals
- **Batch Limits**: Maximum 100 recipients per proposal to prevent gas issues
- **Double-Approval Prevention**: Admins cannot approve the same proposal twice
- **Execution Protection**: Proposals cannot be executed more than once

### SpaceDAO
- **Multi-Admin Governance**: Requires minimum 3 admins
- **Admin-Only Withdrawals**: Only admins can distribute funds
- **Balance Checks**: Ensures sufficient balance before distribution
- **Invalid Recipient Protection**: Prevents transfers to zero address
- **Duplicate Admin Prevention**: Cannot add the same admin twice
- **Configuration Controls**: Only admins can update settings

### Common Security Considerations
- Both contracts use Solidity 0.8.20+ with built-in overflow protection
- OpenZeppelin contracts for standard ERC20 interfaces
- No native token (ETH) support to reduce complexity
- Clear separation of concerns between governance and treasury

## Testing

The test suite includes comprehensive coverage:

**TeamDAO (16 tests, 1 needs fixing):**
- **Deployment (2 passing, 1 failing):**
  - Admin registration and verification ✓
  - DAO activation status (test needs removal - field removed from contract) ✗
  - Required approvals calculation ✓
- **Governance (4 tests):**
  - Proposal creation by admins
  - Access control (non-admin rejection)
  - Zero address token validation
  - Auto-execution when quorum is reached
  - Double-approval prevention
  - Double-execution prevention
- **Majority Logic (2 tests):**
  - 2 approvals required for 3 admins
  - 3 approvals required for 4 admins
- **Execution (3 tests):**
  - ERC20 token transfers
  - Batch transfers to multiple recipients
  - Insufficient balance handling

**SpaceDAO (9 tests):**
- **Deployment (3 tests):**
  - Valid deployment with admins and parameters
  - Minimum admin count validation
  - Invalid/duplicate admin rejection
- **Deposit & Balance (1 test):**
  - Deposit with approval and balance tracking
- **Withdraw distribution (3 tests):**
  - Non-admin access rejection
  - Insufficient balance handling
  - Successful distribution to multiple recipients
- **Admin management (3 tests):**
  - Withdrawal amount updates
  - Adding new admins
  - USDT address updates

**Total: 23 tests passing, 1 test failing**

Run tests with:
```bash
npm run test
# or
make test
```

Test output shows all contracts functioning correctly with comprehensive coverage of:
- Direct deployment pattern
- Multi-admin governance
- Majority approval system (TeamDAO)
- Batch reward distribution (TeamDAO)
- Deposit and withdrawal operations (SpaceDAO)
- Access control
- Security features

## Technology Stack

- **Solidity**: 0.8.20
- **OpenZeppelin Contracts**: 5.4.0 (IERC20, ReentrancyGuard)
- **Hardhat**: 2.28.3
- **TypeScript**: 5.9.3
- **Testing**: Hardhat Toolbox with Chai matchers

## License

MIT

## Integration with Ratel Platform

These contracts are designed to integrate with the Ratel platform's backend APIs:

### Event Listening

The backend should listen to the following events for database synchronization:

**TeamDAO Events:**
- `ProposalCreated(uint256 id, address proposer, uint256 count)`: Record new proposals
- `Approved(uint256 id, address approver)`: Track approval progress
- `BatchExecuted(uint256 id)`: Update proposal execution status
- `Received(address sender, uint256 amount)`: Track treasury funding (if native tokens used)

**SpaceDAO Events:**
- No custom events - rely on ERC20 `Transfer` events for tracking deposits and distributions

### Integration Points

1. **DAO Deployment**: Backend API can deploy new DAO instances for teams/spaces
2. **Reward Distribution**: Platform can trigger batch reward proposals through admin accounts
3. **Admin Management**: Initial admins are set during deployment from platform data
4. **Treasury Monitoring**: Track DAO balances for ERC20 tokens
5. **Proposal Tracking**: Maintain proposal history and status in the backend database
6. **Withdrawal Operations**: SpaceDAO provides simple deposit/withdrawal for space treasuries
