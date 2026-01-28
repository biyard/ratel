/**
 * TeamDAO Service
 *
 * Provides a unified interface for interacting with the TeamDAO smart contract.
 * The TeamDAO contract now includes all reward proposal and execution functionality
 * directly, eliminating the need for separate Factory and Extension contracts.
 *
 * Key features:
 * - Direct DAO deployment via constructor
 * - Integrated reward proposal system with dynamic majority voting
 * - ERC20 token batch transfers
 * - Admin management and permissions
 */

import { ethers } from 'ethers';

import TeamDaoArtifact from './artifacts/TeamDao.json';

const ERC20_ABI = [
  'function balanceOf(address owner) view returns (uint256)',
  'function approve(address spender, uint256 value) returns (bool)',
  'function decimals() view returns (uint8)',
  'function symbol() view returns (string)',
  'function name() view returns (string)',
];

export interface CreateDAOResult {
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

export class TeamDaoService {
  private provider: ethers.AbstractProvider;
  private signer: ethers.JsonRpcSigner | null = null;

  constructor(provider: ethers.AbstractProvider) {
    this.provider = provider;
  }

  async connectWallet() {
    if (!(this.provider instanceof ethers.BrowserProvider)) {
      throw new Error('Browser wallet provider is required for signing.');
    }
    this.signer = await this.provider.getSigner();
  }

  async createDAO(admins: string[]): Promise<CreateDAOResult> {
    if (!this.signer) await this.connectWallet();

    if (admins.length < 3) {
      throw new Error('At least 3 admins are required to create a DAO');
    }

    const TeamDAO = new ethers.ContractFactory(
      TeamDaoArtifact.abi,
      TeamDaoArtifact.bytecode,
      this.signer,
    );

    const dao = await TeamDAO.deploy(admins);
    const receipt = await dao.deploymentTransaction()?.wait();

    if (!receipt) {
      throw new Error('Deployment transaction failed');
    }

    const daoAddress = await dao.getAddress();

    return {
      daoAddress,
      transactionHash: receipt.hash,
    };
  }

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
      this.signer,
    );

    const tx = await dao.proposeBatch(tokenAddress, pairs);
    const receipt = await tx.wait();

    const proposalCreatedEvent = receipt.logs
      .map((log: ethers.Log | ethers.EventLog) => {
        try {
          return dao.interface.parseLog(log);
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

  async approveAndExecuteProposal(
    daoAddress: string,
    proposalId: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.signer,
    );

    const tx = await dao.approveAndExecute(proposalId);
    const receipt = await tx.wait();

    return receipt.hash;
  }

  async getProposalInfo(daoAddress: string, proposalId: number) {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const [token, count, approvals, executed] =
      await dao.getProposalInfo(proposalId);

    return {
      token,
      count: count.toString(),
      approvals: approvals.toString(),
      executed,
    };
  }

  async isDAOAdmin(daoAddress: string, userAddress: string): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    return await dao.checkAdmin(userAddress);
  }

  async isDAOActive(daoAddress: string): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    return await dao.isDaoActive();
  }

  async getRequiredApprovals(daoAddress: string): Promise<number> {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const required = await dao.getRequiredApprovals();
    return Number(required);
  }

  async getDAOInfo(daoAddress: string) {
    const dao = new ethers.Contract(
      daoAddress,
      TeamDaoArtifact.abi,
      this.provider,
    );

    const [isDaoActive, requiredApprovals] = await Promise.all([
      dao.isDaoActive(),
      dao.getRequiredApprovals(),
    ]);

    return {
      daoAddress,
      isDaoActive,
      requiredApprovals: Number(requiredApprovals),
    };
  }
}
