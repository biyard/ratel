
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("Modular DAO System", function () {
  
  async function deploySystemFixture() {
    const [deployer, admin1, admin2, admin3, user, recipient] = await ethers.getSigners();
    const admins = [admin1.address, admin2.address, admin3.address];

    const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
    const spaceDaoLogic = await SpaceDAO.deploy();

    const RewardExtension = await ethers.getContractFactory("RewardExtension");
    const rewardExtLogic = await RewardExtension.deploy();

    const SpaceFactory = await ethers.getContractFactory("SpaceFactory");
    const factory = await SpaceFactory.deploy(
      await spaceDaoLogic.getAddress(),
      await rewardExtLogic.getAddress()
    );

    // Deploy MockToken
    const MockToken = await ethers.getContractFactory("MockToken");
    const token = await MockToken.deploy(); // deployer가 100만 개 가짐

    const tx = await factory.createSpace(admins);
    const receipt = await tx.wait();

    let daoAddress = "";
    let extAddress = "";

    if (receipt && receipt.logs) {
        const filter = factory.filters.SpaceCreated();
        const events = await factory.queryFilter(filter, receipt.blockNumber, receipt.blockNumber);
        if (events.length > 0) {
            const event = events[0] as any;
            daoAddress = event.args[0];
            extAddress = event.args[1];
        }
    }

    const dao = await ethers.getContractAt("SpaceDAO", daoAddress) as any;
    const ext = await ethers.getContractAt("RewardExtension", extAddress) as any;

    return { 
      factory, dao, ext, token, 
      deployer, admin1, admin2, admin3, user, recipient, admins 
    };
  }

  describe("Deployment & Linking", function () {
    it("Should create DAO and Extension correctly", async function () {
      const { dao, ext, admins } = await loadFixture(deploySystemFixture);

      expect(await dao.rewardExtension()).to.equal(await ext.getAddress());
      
      expect(await ext.dao()).to.equal(await dao.getAddress());

      expect(await dao.isExtension(await ext.getAddress())).to.be.true;
    });

    it("Should set admins correctly", async function () {
      const { dao, admins } = await loadFixture(deploySystemFixture);
      
      expect(await dao.isAdmin(admins[0])).to.be.true;
      expect(await dao.isAdmin(admins[1])).to.be.true;
      expect(await dao.isAdmin(admins[2])).to.be.true;
    });
  });

  describe("Governance (Propose & Approve)", function () {
    it("Should allow admin to create proposal", async function () {
      const { ext, admin1, recipient } = await loadFixture(deploySystemFixture);

      const pairs = [{ recipient: recipient.address, amount: 100 }];
      
      await expect(ext.connect(admin1).proposeBatch(ethers.ZeroAddress, pairs))
        .to.emit(ext, "ProposalCreated")
        .withArgs(0, admin1.address, 1);

      const proposal = await ext.getProposalInfo(0);
      expect(proposal.approvals).to.equal(1);
    });

    it("Should NOT allow non-admin to create proposal", async function () {
      const { ext, user, recipient } = await loadFixture(deploySystemFixture);
      const pairs = [{ recipient: recipient.address, amount: 100 }];

      await expect(
        ext.connect(user).proposeBatch(ethers.ZeroAddress, pairs)
      ).to.be.revertedWith("RewardExt: Not a DAO admin");
    });

    it("Should execute automatically when 2/3 quorum is reached", async function () {
      const { dao, ext, admin1, admin2, recipient } = await loadFixture(deploySystemFixture);

      await admin1.sendTransaction({
        to: await dao.getAddress(),
        value: ethers.parseEther("10")
      });

      const amount = ethers.parseEther("1");
      const pairs = [{ recipient: recipient.address, amount: amount }];
      await ext.connect(admin1).proposeBatch(ethers.ZeroAddress, pairs);

      await expect(
        ext.connect(admin2).approveAndExecute(0)
      ).to.changeEtherBalances(
        [dao, recipient],
        [-amount, amount]
      );

      const proposal = await ext.getProposalInfo(0);
      expect(proposal.executed).to.be.true;
    });
  });

  describe("Execution (Funds)", function () {
    
    it("Should transfer ERC20 tokens correctly", async function () {
      const { dao, ext, token, deployer, admin1, admin2, recipient } = await loadFixture(deploySystemFixture);

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);
      
      expect(await token.balanceOf(await dao.getAddress())).to.equal(fundAmount);

      const transferAmount = ethers.parseEther("500");
      const pairs = [{ recipient: recipient.address, amount: transferAmount }];
      
      await ext.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      await ext.connect(admin2).approveAndExecute(0);

      expect(await token.balanceOf(recipient.address)).to.equal(transferAmount);
      expect(await token.balanceOf(await dao.getAddress())).to.equal(ethers.parseEther("500"));
    });

    it("Should handle multiple recipients (Batch)", async function () {
      const { dao, ext, admin1, admin2, user, recipient } = await loadFixture(deploySystemFixture);

      await admin1.sendTransaction({ to: await dao.getAddress(), value: ethers.parseEther("5") });

      const pairs = [
        { recipient: user.address, amount: ethers.parseEther("1") },
        { recipient: recipient.address, amount: ethers.parseEther("2") }
      ];

      await ext.connect(admin1).proposeBatch(ethers.ZeroAddress, pairs);
      await ext.connect(admin2).approveAndExecute(0);

      expect(await ethers.provider.getBalance(user.address)).to.equal(ethers.parseEther("10001")); // 기본 10000 + 1
      expect(await ethers.provider.getBalance(recipient.address)).to.equal(ethers.parseEther("10002")); // 기본 10000 + 2
    });
  });

});

    // "@nomicfoundation/hardhat-toolbox": "^6.1.0",
    // "@openzeppelin/contracts": "^5.4.0",
    // "hardhat": "^3.16.0",
    // "typescript": "^5.9.0",
    // "@types/node": "^20.0.0"