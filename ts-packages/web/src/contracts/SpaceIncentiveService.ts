import { ethers } from 'ethers';

import SpaceIncentiveArtifact from './artifacts/SpaceIncentive.json';

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

export interface CreateSpaceIncentiveResult {
  incentiveAddress: string;
  transactionHash: string;
  deployBlock: number;
}

export class SpaceIncentiveService {
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

  async createSpaceIncentive(
    admins: string[],
    incentiveRecipientCount: number,
    rankingBps = 0,
    mode = 0,
  ): Promise<CreateSpaceIncentiveResult> {
    if (!this.signer) await this.connectWallet();

    if (admins.length < 1) {
      throw new Error('At least 1 admin is required to create an incentive');
    }
    if (
      !Number.isFinite(incentiveRecipientCount) ||
      incentiveRecipientCount <= 0
    ) {
      throw new Error('Incentive recipient count must be greater than 0');
    }

    const factory = new ethers.ContractFactory(
      SpaceIncentiveArtifact.abi,
      SpaceIncentiveArtifact.bytecode,
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
      incentiveAddress: addr,
      transactionHash: txHash,
      deployBlock,
    };
  }

  async getIncentiveRecipientCount(incentiveAddress: string): Promise<number> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const config = await incentive.getIncentiveDistributionConfig();
    const raw = config?.numOfTargets ?? config?.[1] ?? 0;
    return Number(raw);
  }

  async getIncentiveDistributionConfig(incentiveAddress: string): Promise<{
    mode: number;
    numOfTargets: number;
    rankingBps: number;
  }> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const config = await incentive.getIncentiveDistributionConfig();
    const mode = Number(config?.mode ?? config?.[0] ?? 0);
    const numOfTargets = Number(config?.numOfTargets ?? config?.[1] ?? 0);
    const rankingBps = Number(config?.rankingBps ?? config?.[2] ?? 0);
    return { mode, numOfTargets, rankingBps };
  }

  async setIncentiveRecipientCount(
    incentiveAddress: string,
    count: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();
    if (!Number.isFinite(count) || count <= 0) {
      throw new Error('Incentive recipient count must be greater than 0');
    }
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await incentive.setIncentiveRecipientCount(count);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async setIncentiveRankingBps(
    incentiveAddress: string,
    rankingBps: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();
    if (!Number.isFinite(rankingBps) || rankingBps < 0 || rankingBps > 10000) {
      throw new Error('Ranking ratio must be between 0 and 100');
    }
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await incentive.setIncentiveRankingBps(rankingBps);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async selectIncentiveRecipients(
    incentiveAddress: string,
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
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );
    const tx = await incentive.selectIncentiveRecipients(candidates, scores);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async getIncentiveRecipients(incentiveAddress: string): Promise<string[]> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const addresses = await incentive.getIncentiveRecipients();
    return Array.isArray(addresses) ? addresses : [];
  }

  async isIncentiveRecipient(
    incentiveAddress: string,
    account: string,
  ): Promise<boolean> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    return Boolean(await incentive.isIncentiveRecipient(account));
  }

  async isIncentiveClaimed(
    incentiveAddress: string,
    account: string,
  ): Promise<boolean> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    return Boolean(await incentive.isIncentiveClaimed(account));
  }

  async getIncentiveAmount(
    incentiveAddress: string,
    token: string,
  ): Promise<bigint> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.provider,
    );
    const amount = await incentive.getIncentiveAmount(token);
    return BigInt(amount);
  }

  async spaceDeposit(
    incentiveAddress: string,
    amount: string | bigint,
    decimals = 6,
    autoApprove = true,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const usdtAddress: string = await incentive.getUsdt();
    const token = new ethers.Contract(usdtAddress, ERC20_ABI, this.signer);

    const value =
      typeof amount === 'bigint' ? amount : ethers.parseUnits(amount, decimals);

    if (autoApprove) {
      const approveTx = await token.approve(incentiveAddress, value);
      await approveTx.wait();
    }

    const tx = await incentive.deposit(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async spaceDistributeWithdrawal(
    incentiveAddress: string,
    recipients: string[],
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const tx = await incentive.distributeWithdrawal(recipients);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async distribute(
    incentiveAddress: string,
    token: string,
    recipients: string[],
    value: bigint,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const tx = await incentive.distribute(token, recipients, value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async claimIncentive(
    incentiveAddress: string,
    token: string,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      INCENTIVE_DISTRIBUTION_ABI,
      this.signer,
    );

    const tx = await incentive.claimIncentive(token);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async proposeShareWithdrawal(
    incentiveAddress: string,
    amount: string | bigint,
    decimals = 6,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const value =
      typeof amount === 'bigint' ? amount : ethers.parseUnits(amount, decimals);

    const tx = await incentive.proposeShareWithdrawal(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async approveShareWithdrawal(
    incentiveAddress: string,
    id: number,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const tx = await incentive.approveShareWithdrawal(id);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async getShareWithdrawProposalCount(
    incentiveAddress: string,
  ): Promise<number> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    const count = await incentive.getShareWithdrawProposalCount();
    return Number(count);
  }

  async getShareWithdrawProposal(
    incentiveAddress: string,
    id: number,
  ): Promise<{
    proposer: string;
    amount: string;
    approvals: string;
    executed: boolean;
  }> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    const [proposer, amount, approvals, executed] =
      await incentive.getShareWithdrawProposal(id);
    return {
      proposer,
      amount: amount.toString(),
      approvals: approvals.toString(),
      executed,
    };
  }

  async isShareWithdrawApproved(
    incentiveAddress: string,
    id: number,
    approver: string,
  ): Promise<boolean> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    return incentive.isShareWithdrawApproved(id, approver);
  }

  async getDepositorCount(incentiveAddress: string): Promise<number> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    const count = await incentive.getDepositorCount();
    return Number(count);
  }

  async getDepositorDeposit(
    incentiveAddress: string,
    depositor: string,
  ): Promise<string> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    const value = await incentive.getDepositorDeposit(depositor);
    return value.toString();
  }

  async getAvailableShare(
    incentiveAddress: string,
    depositor: string,
  ): Promise<string> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );
    const value = await incentive.getAvailableShare(depositor);
    return value.toString();
  }

  async getSpaceBalance(incentiveAddress: string): Promise<string> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );

    const balance = await incentive.getBalance();
    return balance.toString();
  }

  async setSpaceWithdrawalAmount(
    incentiveAddress: string,
    amount: string | bigint,
    decimals = 6,
  ): Promise<string> {
    if (!this.signer) await this.connectWallet();

    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.signer,
    );

    const value =
      typeof amount === 'bigint' ? amount : ethers.parseUnits(amount, decimals);

    const tx = await incentive.setWithdrawalAmount(value);
    const receipt = await tx.wait();
    return receipt.hash;
  }

  async getSpaceWithdrawalAmount(incentiveAddress: string): Promise<string> {
    const incentive = new ethers.Contract(
      incentiveAddress,
      SpaceIncentiveArtifact.abi,
      this.provider,
    );

    const amount = await incentive.getWithdrawalAmount();
    return amount.toString();
  }
}
