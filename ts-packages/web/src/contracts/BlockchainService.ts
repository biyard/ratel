// services/BlockchainService.ts
import { ethers } from 'ethers';

import SpaceFactoryArtifact from './artifacts/SpaceFactory.json';
import SpaceDAOArtifact from './artifacts/SpaceDAO.json';
import RewardExtensionArtifact from './artifacts/RewardExtension.json';

const FACTORY_ADDRESS = process.env.NEXT_PUBLIC_FACTORY_ADDRESS || '';

export class BlockchainService {
  private provider: ethers.BrowserProvider;
  private signer: ethers.JsonRpcSigner | null = null;

  constructor(provider: ethers.BrowserProvider) {
    this.provider = provider;
  }

  async connectWallet() {
    this.signer = await this.provider.getSigner();
  }

  async createDAO(admins: string[]) {
    if (!this.signer) await this.connectWallet();

    const factory = new ethers.Contract(
      FACTORY_ADDRESS,
      SpaceFactoryArtifact.abi,
      this.signer,
    );

    const tx = await factory.createSpace(admins);
    await tx.wait();

    return { daoAddress: '0x...', extAddress: '0x...' };
  }

  async proposeReward(daoAddress: string, token: string, pairs: any[]) {
    if (!this.signer) await this.connectWallet();

    const dao = new ethers.Contract(
      daoAddress,
      SpaceDAOArtifact.abi,
      this.provider,
    );

    const extAddress = await dao.rewardExtension();

    const ext = new ethers.Contract(
      extAddress,
      RewardExtensionArtifact.abi,
      this.signer,
    );

    const tx = await ext.proposeBatch(token, pairs);
    await tx.wait();

    return tx.hash;
  }

  async getDAOInfo(daoAddress: string) {
    const dao = new ethers.Contract(
      daoAddress,
      SpaceDAOArtifact.abi,
      this.provider,
    );
    const admins = await dao.getAdmins(); // (가정: getter 함수가 있다면)
    return { admins };
  }
}
