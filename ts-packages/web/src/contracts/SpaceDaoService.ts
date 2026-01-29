import { ethers } from 'ethers';

import SpaceDaoArtifact from './artifacts/SpaceDao.json';

const ERC20_ABI = [
  'function balanceOf(address owner) view returns (uint256)',
  'function approve(address spender, uint256 value) returns (bool)',
  'function decimals() view returns (uint8)',
  'function symbol() view returns (string)',
  'function name() view returns (string)',
];

export interface CreateSpaceDAOResult {
  daoAddress: string;
  transactionHash: string;
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
