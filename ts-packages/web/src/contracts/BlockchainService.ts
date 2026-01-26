// services/BlockchainService.ts
import { ethers } from 'ethers';

import TeamDaoFactoryArtifact from './artifacts/TeamDaoFactory.json';
import TeamDaoArtifact from './artifacts/TeamDAO.json';
import RewardExtensionArtifact from './artifacts/RewardExtension.json';
import SpaceDaoArtifact from './artifacts/SpaceDAO.json';

const FACTORY_ADDRESS = import.meta.env.VITE_FACTORY_ADDRESS || '';

// ERC20 ABI for token balance checking
const ERC20_ABI = [
  'function balanceOf(address owner) view returns (uint256)',
  'function approve(address spender, uint256 value) returns (bool)',
  'function decimals() view returns (uint8)',
  'function symbol() view returns (string)',
  'function name() view returns (string)',
];

export interface CreateDAOResult {
  daoAddress: string;
  rewardExtensionAddress: string;
  transactionHash: string;
}

export interface CreateSpaceDAOResult {
  daoAddress: string;
  transactionHash: string;
}

export interface TokenBalance {
  balance: string;
  decimals: number;
  symbol: string;
  name: string;
  formatted: string;
}

export interface TransferPair {
  recipient: string;
  amount: string;
}

export class BlockchainService {
  private provider: ethers.BrowserProvider;
  private signer: ethers.JsonRpcSigner | null = null;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async connectWallet() {
    this.signer = await this.provider.getSigner();
  }

  /**
   * Create a new DAO with the specified admins
   * @param admins Array of admin addresses (minimum 3 required)
   * @returns DAO address, reward extension address, and transaction hash
   */
  async createDAO(admins: string[]): Promise<CreateDAOResult> {
    if (!this.signer) await this.connectWallet();

    if (admins.length < 3) {
      throw new Error('At least 3 admins are required to create a DAO');
    }

    const factory = new ethers.Contract(
      FACTORY_ADDRESS,
      TeamDaoFactoryArtifact.abi,
      this.signer,
    );

    // Call createSpace and wait for transaction
    const tx = await factory.createSpace(admins);
    const receipt = await tx.wait();

    // Parse SpaceCreated event to get DAO and extension addresses
    const spaceCreatedEvent = receipt.logs
      .map((log: ethers.Log | ethers.EventLog) => {
        try {
          return factory.interface.parseLog(log);
        } catch {
          return null;
        }
      })
      .find(
        (event: ethers.LogDescription | null): event is ethers.LogDescription =>
          event?.name === 'SpaceCreated',
      );

    if (!spaceCreatedEvent) {
      throw new Error('SpaceCreated event not found in transaction receipt');
    }

    return {
      daoAddress: spaceCreatedEvent.args.dao,
      rewardExtensionAddress: spaceCreatedEvent.args.rewardExtension,
      transactionHash: receipt.hash,
    };
  }

  /**
   * Deploy SpaceDAO directly (no factory).
   * @param admins Array of admin addresses (minimum 3 required)
   * @param usdtAddress ERC20 token address used by SpaceDAO
   * @param withdrawalAmount Amount per recipient (string in human units or raw bigint)
   * @param decimals Token decimals (default 6)
   */
  async createSpaceDAO(
    admins: string[],
    usdtAddress: string,
    withdrawalAmount: string | bigint,
    decimals = 6,
  ): Promise<CreateSpaceDAOResult> {
    if (!this.signer) await this.connectWallet();

    if (admins.length < 3) {
      throw new Error('At least 3 admins are required to create a DAO');
    }
    if (!usdtAddress) {
      throw new Error('USDT address is required');
    }

    const amount =
      typeof withdrawalAmount === 'bigint'
        ? withdrawalAmount
        : ethers.parseUnits(withdrawalAmount, decimals);

    const factory = new ethers.ContractFactory(
      SpaceDaoArtifact.abi,
      SpaceDaoArtifact.bytecode,
      this.signer!,
    );

    const contract = await factory.deploy(admins, usdtAddress, amount);
    await contract.waitForDeployment();

    const addr = await contract.getAddress();
    const txHash = contract.deploymentTransaction()?.hash ?? '';

    return {
      daoAddress: addr,
      transactionHash: txHash,
    };
  }

  /**
   * Deposit USDT into SpaceDAO.
   * @param daoAddress SpaceDAO address
   * @param amount Amount to deposit (string in human units or raw bigint)
   * @param decimals Token decimals (default 6)
   * @param autoApprove Whether to approve before deposit (default true)
   */
  async spaceDeposit(
    daoAddress: string,
    amount: string | bigint,
    decimals = 6,
    autoApprove = true,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.signer,
    );

    const usdtAddress: string = await dao.getUsdt();
    const token = new ethers.Contract(usdtAddress, ERC20_ABI, this.signer);

    const value =
      typeof amount === 'bigint' ? amount : ethers.parseUnits(amount, decimals);

    if (autoApprove) {
      const approveTx = await token.approve(daoAddress, value);
      await approveTx.wait();
    }

    const tx = await dao.deposit(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  /**
   * Distribute withdrawal amount to recipients (admin only).
   */
  async spaceDistributeWithdrawal(
    daoAddress: string,
    recipients: string[],
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.signer,
    );

    const tx = await dao.distributeWithdrawal(recipients);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  /**
   * Get SpaceDAO USDT balance.
   */
  async getSpaceBalance(daoAddress: string): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );

    const balance = await dao.getBalance();
    return balance.toString();
  }

  /**
   * Update withdrawal amount (admin only).
   */
  async setSpaceWithdrawalAmount(
    daoAddress: string,
    amount: string | bigint,
    decimals = 6,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.signer,
    );

    const value =
      typeof amount === 'bigint' ? amount : ethers.parseUnits(amount, decimals);

    const tx = await dao.setWithdrawalAmount(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  /**
   * Get withdrawal amount.
   */
  async getSpaceWithdrawalAmount(daoAddress: string): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );

    const amount = await dao.getWithdrawalAmount();
    return amount.toString();
  }

  /**
   * Get token balance for a DAO
   * @param daoAddress DAO address
   * @param tokenAddress ERC20 token address
   * @returns Token balance information
   */
  async getDAOTokenBalance(
    daoAddress: string,
    tokenAddress: string,
  ): Promise<TokenBalance> {
    const tokenContract = new ethers.Contract(
      tokenAddress,
      ERC20_ABI,
      this.provider,
    );

    const [balance, decimals, symbol, name] = await Promise.all([
      tokenContract.balanceOf(daoAddress),
      tokenContract.decimals(),
      tokenContract.symbol(),
      tokenContract.name(),
    ]);

    const formatted = ethers.formatUnits(balance, decimals);

    return {
      balance: balance.toString(),
      decimals,
      symbol,
      name,
      formatted,
    };
  }

  /**
   * Get token balances for a DAO (토큰 목록은 백엔드 API에서 제공)
   * @param daoAddress DAO address
   * @param tokenAddresses Array of ERC20 token addresses to check
   * @returns Token balances
   */
  async getDAOBalances(
    daoAddress: string,
    tokenAddresses: string[],
  ): Promise<TokenBalance[]> {
    if (tokenAddresses.length === 0) {
      return [];
    }

    const tokenBalances = await Promise.all(
      tokenAddresses.map((tokenAddress) =>
        this.getDAOTokenBalance(daoAddress, tokenAddress),
      ),
    );

    return tokenBalances;
  }

  /**
   * Propose a batch reward distribution (토큰 분배 요청)
   * @param daoAddress DAO address
   * @param tokenAddress ERC20 token address (use address(0) for native token)
   * @param pairs Array of recipient and amount pairs
   * @returns Transaction hash and proposal ID
   */
  async proposeReward(
    daoAddress: string,
    tokenAddress: string,
    pairs: TransferPair[],
  ): Promise<{ transactionHash: string; proposalId: number }> {
    if (!this.signer) await this.connectWallet();

    if (pairs.length === 0 || pairs.length > 100) {
      throw new Error('Invalid pairs: must have 1-100 pairs');
    }

    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const extAddress = await dao.rewardExtension();

    const ext = new ethers.Contract(
      extAddress,
      RewardExtensionArtifact.abi,
      this.signer,
    );

    const tx = await ext.proposeBatch(tokenAddress, pairs);
    const receipt = await tx.wait();

    // Parse ProposalCreated event to get proposal ID
    const proposalCreatedEvent = receipt.logs
      .map((log: ethers.Log | ethers.EventLog) => {
        try {
          return ext.interface.parseLog(log);
        } catch {
          return null;
        }
      })
      .find(
        (event: ethers.LogDescription | null): event is ethers.LogDescription =>
          event?.name === 'ProposalCreated',
      );

    const proposalId = proposalCreatedEvent
      ? Number(proposalCreatedEvent.args.id)
      : 0;

    return {
      transactionHash: receipt.hash,
      proposalId,
    };
  }

  /**
   * Approve and execute a reward proposal
   * @param daoAddress DAO address
   * @param proposalId Proposal ID
   * @returns Transaction hash
   */
  async approveAndExecuteProposal(
    daoAddress: string,
    proposalId: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const extAddress = await dao.rewardExtension();

    const ext = new ethers.Contract(
      extAddress,
      RewardExtensionArtifact.abi,
      this.signer,
    );

    const tx = await ext.approveAndExecute(proposalId);
    const receipt = await tx.wait();

    return receipt.hash;
  }

  /**
   * Get proposal information
   * @param daoAddress DAO address
   * @param proposalId Proposal ID
   * @returns Proposal details
   */
  async getProposalInfo(daoAddress: string, proposalId: number) {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const extAddress = await dao.rewardExtension();

    const ext = new ethers.Contract(
      extAddress,
      RewardExtensionArtifact.abi,
      this.provider,
    );

    const [token, count, approvals, executed] =
      await ext.getProposalInfo(proposalId);

    return {
      token,
      count: count.toString(),
      approvals: approvals.toString(),
      executed,
    };
  }

  /**
   * Check if an address is a DAO admin
   * @param daoAddress DAO address
   * @param userAddress User address to check
   * @returns Whether the user is an admin
   */
  async isDAOAdmin(daoAddress: string, userAddress: string): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    return await dao.checkAdmin(userAddress);
  }

  /**
   * Check if DAO is active
   * @param daoAddress DAO address
   * @returns Whether the DAO is active
   */
  async isDAOActive(daoAddress: string): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    return await dao.isDaoActive();
  }

  /**
   * Get DAO information (legacy method)
   * @param daoAddress DAO address
   * @returns DAO address
   * @deprecated Use isDAOAdmin or isDAOActive instead
   */
  async getDAOInfo(daoAddress: string) {
    // Note: The ABI doesn't have a getAdmins() method
    // You can only check individual addresses with checkAdmin()
    // or access the admins array by index
    return { daoAddress };
  }
}
