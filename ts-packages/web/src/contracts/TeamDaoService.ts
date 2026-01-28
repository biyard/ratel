import { ethers } from 'ethers';

import TeamDaoFactoryArtifact from './artifacts/TeamDaoFactory.json';
import TeamDaoArtifact from './artifacts/TeamDao.json';
import RewardExtensionArtifact from './artifacts/RewardExtension.json';

const FACTORY_ADDRESS = import.meta.env.VITE_FACTORY_ADDRESS || '';

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

    const factory = new ethers.Contract(
      FACTORY_ADDRESS,
      TeamDaoFactoryArtifact.abi,
      this.signer,
    );

    const tx = await factory.createSpace(admins);
    const receipt = await tx.wait();

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

  async getDAOInfo(daoAddress: string) {
    return { daoAddress };
  }
}
