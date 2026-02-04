import { ethers } from 'ethers';

import SpaceDaoArtifact from './artifacts/SpaceDao.json';

const ERC20_ABI = [
  'function balanceOf(address owner) view returns (uint256)',
  'function approve(address spender, uint256 value) returns (bool)',
  'function decimals() view returns (uint8)',
  'function symbol() view returns (string)',
  'function name() view returns (string)',
];

const INCENTIVE_DISTRIBUTION_ABI = [
  'function getIncentiveDistributionConfig() view returns (tuple(uint8 mode,uint256 numOfTargets,uint16 rankingBps))',
  'function setIncentiveRecipientCount(uint256 numOfTargets)',
  'function setIncentiveRankingBps(uint16 rankingBps)',
  'function selectIncentiveRecipients(address[] candidates,uint256[] scores) returns (address[])',
  'function getIncentiveRecipients() view returns (address[])',
  'function getIncentiveAmount(address token) view returns (uint256)',
  'function isIncentiveRecipient(address account) view returns (bool)',
  'function isIncentiveClaimed(address account) view returns (bool)',
  'function claimIncentive(address token)',
];

export interface CreateSpaceDAOResult {
  daoAddress: string;
  transactionHash: string;
  deployBlock: number;
}

export class SpaceDaoService {
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

  async createSpaceDAO(
    admins: string[],
    incentiveRecipientCount: number,
    rankingBps = 0,
    mode = 0,
  ): Promise<CreateSpaceDAOResult> {
    if (!this.signer) await this.connectWallet();

    if (admins.length < 1) {
      throw new Error('At least 1 admin is required to create a DAO');
    }
    if (
      !Number.isFinite(incentiveRecipientCount) ||
      incentiveRecipientCount <= 0
    ) {
      throw new Error('Incentive recipient count must be greater than 0');
    }

    const factory = new ethers.ContractFactory(
      SpaceDaoArtifact.abi,
      SpaceDaoArtifact.bytecode,
      this.signer!,
    );

    const contract = await factory.deploy(admins, {
      mode,
      numOfTargets: incentiveRecipientCount,
      rankingBps,
    });
    await contract.waitForDeployment();

    const addr = await contract.getAddress();
    const txHash = contract.deploymentTransaction()?.hash ?? '';
    const deployReceipt = await contract.deploymentTransaction()?.wait();
    const deployBlock = deployReceipt?.blockNumber ?? 0;

    return {
      daoAddress: addr,
      transactionHash: txHash,
      deployBlock,
    };
  }

  async getIncentiveRecipientCount(daoAddress: string): Promise<number> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const config = await dao.getIncentiveDistributionConfig();
    const raw = config?.numOfTargets ?? config?.[1] ?? 0;
    return Number(raw);
  }

  async getIncentiveDistributionConfig(daoAddress: string): Promise<{
    mode: number;
    numOfTargets: number;
    rankingBps: number;
  }> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const config = await dao.getIncentiveDistributionConfig();
    const mode = Number(config?.mode ?? config?.[0] ?? 0);
    const numOfTargets = Number(config?.numOfTargets ?? config?.[1] ?? 0);
    const rankingBps = Number(config?.rankingBps ?? config?.[2] ?? 0);
    return { mode, numOfTargets, rankingBps };
  }

  async setIncentiveRecipientCount(
    daoAddress: string,
    count: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();
    if (!Number.isFinite(count) || count <= 0) {
      throw new Error('Incentive recipient count must be greater than 0');
    }
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await dao.setIncentiveRecipientCount(count);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async setIncentiveRankingBps(
    daoAddress: string,
    rankingBps: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();
    if (!Number.isFinite(rankingBps) || rankingBps < 0 || rankingBps > 10000) {
      throw new Error('Ranking ratio must be between 0 and 100');
    }
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await dao.setIncentiveRankingBps(rankingBps);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async selectIncentiveRecipients(
    daoAddress: string,
    candidates: string[],
    scores: number[],
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();
    if (candidates.length === 0) {
      throw new Error('Candidates are required to select incentives');
    }
    if (scores.length !== candidates.length) {
      throw new Error('Scores length must match candidates length');
    }
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await dao.selectIncentiveRecipients(candidates, scores);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async getIncentiveRecipients(daoAddress: string): Promise<string[]> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const addresses = await dao.getIncentiveRecipients();
    return Array.isArray(addresses) ? addresses : [];
  }

  async isIncentiveRecipient(
    daoAddress: string,
    account: string,
  ): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    return Boolean(await dao.isIncentiveRecipient(account));
  }

  async isIncentiveClaimed(
    daoAddress: string,
    account: string,
  ): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    return Boolean(await dao.isIncentiveClaimed(account));
  }

  async getIncentiveAmount(daoAddress: string, token: string): Promise<bigint> {
    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const amount = await dao.getIncentiveAmount(token);
    return BigInt(amount);
  }

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

  async distribute(
    daoAddress: string,
    token: string,
    recipients: string[],
    value: bigint,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.signer,
    );

    const tx = await dao.distribute(token, recipients, value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async claimIncentive(daoAddress: string, token: string): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );

    const tx = await dao.claimIncentive(token);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async proposeShareWithdrawal(
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

    const tx = await dao.proposeShareWithdrawal(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async approveShareWithdrawal(
    daoAddress: string,
    id: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.signer,
    );

    const tx = await dao.approveShareWithdrawal(id);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async getShareWithdrawProposalCount(daoAddress: string): Promise<number> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    const count = await dao.getShareWithdrawProposalCount();
    return Number(count);
  }

  async getShareWithdrawProposal(
    daoAddress: string,
    id: number,
  ): Promise<{
    proposer: string;
    amount: string;
    approvals: string;
    executed: boolean;
  }> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    const [proposer, amount, approvals, executed] =
      await dao.getShareWithdrawProposal(id);
    return {
      proposer,
      amount: amount.toString(),
      approvals: approvals.toString(),
      executed,
    };
  }

  async isShareWithdrawApproved(
    daoAddress: string,
    id: number,
    approver: string,
  ): Promise<boolean> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    return dao.isShareWithdrawApproved(id, approver);
  }

  async getDepositorCount(daoAddress: string): Promise<number> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    const count = await dao.getDepositorCount();
    return Number(count);
  }

  async getDepositorDeposit(
    daoAddress: string,
    depositor: string,
  ): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    const value = await dao.getDepositorDeposit(depositor);
    return value.toString();
  }

  async getAvailableShare(
    daoAddress: string,
    depositor: string,
  ): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );
    const value = await dao.getAvailableShare(depositor);
    return value.toString();
  }

  async getSpaceBalance(daoAddress: string): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );

    const balance = await dao.getBalance();
    return balance.toString();
  }

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

  async getSpaceWithdrawalAmount(daoAddress: string): Promise<string> {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDaoArtifact.abi,
      this.provider,
    );

    const amount = await dao.getWithdrawalAmount();
    return amount.toString();
  }
}
