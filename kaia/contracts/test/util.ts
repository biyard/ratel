import { assert, expect } from "chai";
import { ethers as e } from "ethers";
import hre, { ethers, network, web3 } from "hardhat";
import { Artifact } from "hardhat/types";

export const ZeroAddress = "0x0000000000000000000000000000000000000000";

export type ContractInformation = e.Contract;

export type UserInformation = {
  address: string;
  privateKey: string;
};

export class TestingUtils {
  owner: string;
  ownerKey: string;
  feepayer: string;
  feepayerKey: string;

  constructor() {
    const path = `${__dirname}/../.env`;
    require("dotenv").config({ path });

    this.owner = process.env.CLI_CONTRACT_OWNER_ADDR ?? "";
    this.ownerKey = process.env.CLI_CONTRACT_OWNER_KEY ?? "";
    this.feepayer = process.env.CLI_FEEPAYER_ADDR ?? "";
    this.feepayerKey = process.env.CLI_FEEPAYER_KEY ?? "";
  }

  isAddress = (addr: string): boolean => {
    return /^0x[a-fA-F0-9]{40}$/.test(addr);
  };

  load = (contractName: string): Artifact => {
    return hre.artifacts.readArtifactSync(contractName);
  };

  fund = async (address: string, ethAmount: number): Promise<e.Contract> => {
    await network.provider.send("hardhat_setBalance", [
      address,
      `0x${(BigInt(ethAmount) * 10n ** 18n).toString(16)}`,
    ]);
  };

  loadContract = async (
    contractName: string,
    addr: string
  ): Promise<e.Contract> => {
    return ethers.getContractAt(contractName, addr);
  };

  deploy = async (contractName: string, ...args: any): Promise<e.Contract> => {
    const contract = await ethers.getContractFactory(contractName);
    return await contract.deploy(...args);
  };

  execute = async (
    data: ContractInformation,
    method: string,
    ...args: any
  ): Promise<any> => {
    return data[method](...args, {
      gasPrice: 250000000000,
      gasLimit: 100000000,
    });
  };

  executeOwner = async (
    data: ContractInformation,
    method: string,
    ...args: any
  ): Promise<any> => {
    const provider = ethers.provider;
    const wallet = new e.Wallet(this.ownerKey, provider);
    const contract = data.connect(wallet);

    return this.execute(contract, method, ...args);
  };

  executeFeePayer = async (
    data: ContractInformation,
    method: string,
    ...args: any
  ): Promise<any> => {
    const provider = ethers.provider;

    const wallet = new e.Wallet(this.feepayerKey, provider);
    const contract = data.connect(wallet);

    return this.execute(contract, method, ...args);
  };

  executePayable = async (
    data: ContractInformation,
    feeValue: number,
    method: string,
    ...args: any
  ): Promise<any> => {
    return data[method](...args, {
      value: ethers.utils.parseEther(`${feeValue}`),
    });
  };

  executePayableFeePayer = async (
    data: ContractInformation,
    feeValue: number,
    method: string,
    ...args: any
  ): Promise<any> => {
    const contract = data.connect(await ethers.getSigner(this.feepayer));

    return this.executePayable(contract, feeValue, method, ...args);
  };

  call = async (
    data: ContractInformation,
    method: string,
    ...args: any
  ): Promise<any> => {
    return data[method](...args);
  };

  // existsEvent = (tx: any, eventName: string): any => {
  //   assert.exists(tx.events[eventName]);

  //   return tx.events[eventName].returnValues;
  // };
}

export class Utils {
  gasPrice: number;
  gasLimit: number;
  nullAddress: string;

  constructor() {
    this.gasPrice = 250000000000;
    this.gasLimit = 30000000;
    this.nullAddress = "0x0000000000000000000000000000000000000000";
  }

  isAddress = (addr: string): boolean => {
    return /^0x[a-fA-F0-9]{40}$/.test(addr);
  };

  loadArtifact = (contractName: string): Artifact => {
    return hre.artifacts.readArtifactSync(contractName);
  };

  loadContract = async (
    contractName: string,
    addr: string
  ): Promise<e.Contract> => {
    return ethers.getContractAt(contractName, addr);
  };

  deploy = async (contractName: string, ...args: any): Promise<e.Contract> => {
    // const contract = await ethers.getContractFactory(contractName);
    // return await contract.deploy(...args);

    return ethers.deployContract(contractName, args);
  };

  sendBalance = async (addr: string, amount: string): Promise<any> => {
    await network.provider.send("hardhat_setBalance", [addr, amount]);
  };

  execute = async (
    data: e.Contract,
    method: string,
    ...args: any
  ): Promise<any> => {
    return data[method](...args, {
      gasPrice: this.gasPrice,
      gasLimit: this.gasLimit,
    });
  };

  executeSigner = async (
    signer: string,
    data: e.Contract,
    method: string,
    ...args: any
  ): Promise<any> => {
    const contract = data.connect(await ethers.getSigner(signer));
    return await contract[method](...args, {
      gasPrice: this.gasPrice,
      gasLimit: this.gasLimit,
    });
  };

  executeImpersonated = async (
    signer: string,
    data: e.Contract,
    method: string,
    ...args: any
  ): Promise<any> => {
    const contract = data.connect(await ethers.getImpersonatedSigner(signer));
    return await contract[method](...args, {
      gasPrice: this.gasPrice,
      gasLimit: this.gasLimit,
    });
  };

  setGasPrice = (price: number) => {
    this.gasPrice = price;
  };

  getSigner = async (signer: string) => {
    return ethers.getSigner(signer);
  };

  getAllSigner = async () => {
    return ethers.getSigners();
  };

  createAccount = () => {
    const w = e.Wallet.createRandom();

    return w.address;
  };

  createWallet = () => {
    const w = e.Wallet.createRandom();

    return w;
  };
}
