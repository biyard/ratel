import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("TeamDAO System", function () {
  async function deployTeamDAOFixture() {
    const [deployer, admin1, admin2, admin3, admin4, user, recipient] =
      await ethers.getSigners();
    const admins = [admin1.address, admin2.address, admin3.address];

    const TeamDAO = await ethers.getContractFactory("TeamDAO");
    const dao = await TeamDAO.deploy(admins);
    await dao.waitForDeployment();

    const MockToken = await ethers.getContractFactory("MockToken");
    const token = await MockToken.deploy();
    await token.waitForDeployment();

    return {
      dao,
      token,
      deployer,
      admin1,
      admin2,
      admin3,
      admin4,
      user,
      recipient,
      admins,
    };
  }

  describe("Deployment", function () {
    it("Should set admins correctly", async function () {
      const { dao, admins } = await loadFixture(deployTeamDAOFixture);

      expect(await dao.isAdmin(admins[0])).to.be.true;
      expect(await dao.isAdmin(admins[1])).to.be.true;
      expect(await dao.isAdmin(admins[2])).to.be.true;
    });

    it("Should set DAO as active", async function () {
      const { dao } = await loadFixture(deployTeamDAOFixture);

      expect(await dao.isDaoActive()).to.be.true;
    });

    it("Should calculate required approvals correctly for 3 admins", async function () {
      const { dao } = await loadFixture(deployTeamDAOFixture);

      expect(await dao.getRequiredApprovals()).to.equal(2);
    });
  });

  describe("Governance (Propose & Approve)", function () {
    it("Should allow admin to create proposal", async function () {
      const { dao, token, admin1, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const pairs = [{ recipient: recipient.address, amount: 100 }];

      await expect(
        dao.connect(admin1).proposeBatch(await token.getAddress(), pairs)
      )
        .to.emit(dao, "ProposalCreated")
        .withArgs(0, admin1.address, 1);

      const proposal = await dao.getProposalInfo(0);
      expect(proposal.approvals).to.equal(1);
    });

    it("Should NOT allow non-admin to create proposal", async function () {
      const { dao, token, user, recipient } = await loadFixture(
        deployTeamDAOFixture
      );
      const pairs = [{ recipient: recipient.address, amount: 100 }];

      await expect(
        dao.connect(user).proposeBatch(await token.getAddress(), pairs)
      ).to.be.revertedWith("TeamDAO: Not an admin");
    });

    it("Should NOT allow zero address for token", async function () {
      const { dao, admin1, recipient } = await loadFixture(
        deployTeamDAOFixture
      );
      const pairs = [{ recipient: recipient.address, amount: 100 }];

      await expect(
        dao.connect(admin1).proposeBatch(ethers.ZeroAddress, pairs)
      ).to.be.revertedWith("TeamDAO: Token address cannot be zero");
    });

    it("Should execute automatically when majority (2/3) is reached", async function () {
      const { dao, token, admin1, admin2, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);

      const amount = ethers.parseEther("100");
      const pairs = [{ recipient: recipient.address, amount: amount }];
      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      const balanceBefore = await token.balanceOf(recipient.address);

      await expect(dao.connect(admin2).approveAndExecute(0))
        .to.emit(dao, "Approved")
        .withArgs(0, admin2.address)
        .to.emit(dao, "BatchExecuted")
        .withArgs(0);

      const balanceAfter = await token.balanceOf(recipient.address);
      expect(balanceAfter - balanceBefore).to.equal(amount);

      const proposal = await dao.getProposalInfo(0);
      expect(proposal.executed).to.be.true;
      expect(proposal.approvals).to.equal(2);
    });

    it("Should NOT execute if already approved by same admin", async function () {
      const { dao, token, admin1, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const pairs = [{ recipient: recipient.address, amount: 100 }];
      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      await expect(
        dao.connect(admin1).approveAndExecute(0)
      ).to.be.revertedWith("TeamDAO: Already approved");
    });

    it("Should NOT execute proposal twice", async function () {
      const { dao, token, admin1, admin2, admin3, recipient } =
        await loadFixture(deployTeamDAOFixture);

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);

      const pairs = [{ recipient: recipient.address, amount: 100 }];
      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);
      await dao.connect(admin2).approveAndExecute(0);

      await expect(
        dao.connect(admin3).approveAndExecute(0)
      ).to.be.revertedWith("TeamDAO: Already executed");
    });
  });

  describe("Majority Logic", function () {
    it("Should require 2 approvals for 3 admins", async function () {
      const { dao, token, admin1, admin2, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      expect(await dao.getRequiredApprovals()).to.equal(2);

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);

      const pairs = [{ recipient: recipient.address, amount: 100 }];
      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      const proposalBefore = await dao.getProposalInfo(0);
      expect(proposalBefore.executed).to.be.false;
      expect(proposalBefore.approvals).to.equal(1);

      await dao.connect(admin2).approveAndExecute(0);

      const proposalAfter = await dao.getProposalInfo(0);
      expect(proposalAfter.executed).to.be.true;
      expect(proposalAfter.approvals).to.equal(2);
    });

    it("Should require 3 approvals for 4 admins", async function () {
      const [deployer, admin1, admin2, admin3, admin4, user, recipient] =
        await ethers.getSigners();
      const admins = [
        admin1.address,
        admin2.address,
        admin3.address,
        admin4.address,
      ];

      const TeamDAO = await ethers.getContractFactory("TeamDAO");
      const dao = await TeamDAO.deploy(admins);
      await dao.waitForDeployment();

      const MockToken = await ethers.getContractFactory("MockToken");
      const token = await MockToken.deploy();
      await token.waitForDeployment();

      expect(await dao.getRequiredApprovals()).to.equal(3);

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);

      const pairs = [{ recipient: recipient.address, amount: 100 }];
      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      await dao.connect(admin2).approveAndExecute(0);

      let proposal = await dao.getProposalInfo(0);
      expect(proposal.executed).to.be.false;
      expect(proposal.approvals).to.equal(2);

      await dao.connect(admin3).approveAndExecute(0);

      proposal = await dao.getProposalInfo(0);
      expect(proposal.executed).to.be.true;
      expect(proposal.approvals).to.equal(3);
    });
  });

  describe("Execution (ERC20 Transfers)", function () {
    it("Should transfer ERC20 tokens correctly", async function () {
      const { dao, token, admin1, admin2, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const fundAmount = ethers.parseEther("1000");
      await token.transfer(await dao.getAddress(), fundAmount);

      expect(await token.balanceOf(await dao.getAddress())).to.equal(
        fundAmount
      );

      const transferAmount = ethers.parseEther("500");
      const pairs = [{ recipient: recipient.address, amount: transferAmount }];

      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);
      await dao.connect(admin2).approveAndExecute(0);

      expect(await token.balanceOf(recipient.address)).to.equal(
        transferAmount
      );
      expect(await token.balanceOf(await dao.getAddress())).to.equal(
        ethers.parseEther("500")
      );
    });

    it("Should handle multiple recipients (Batch)", async function () {
      const { dao, token, admin1, admin2, user, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const fundAmount = ethers.parseEther("5000");
      await token.transfer(await dao.getAddress(), fundAmount);

      const pairs = [
        { recipient: user.address, amount: ethers.parseEther("1000") },
        { recipient: recipient.address, amount: ethers.parseEther("2000") },
      ];

      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);
      await dao.connect(admin2).approveAndExecute(0);

      expect(await token.balanceOf(user.address)).to.equal(
        ethers.parseEther("1000")
      );
      expect(await token.balanceOf(recipient.address)).to.equal(
        ethers.parseEther("2000")
      );
      expect(await token.balanceOf(await dao.getAddress())).to.equal(
        ethers.parseEther("2000")
      );
    });

    it("Should revert if DAO has insufficient token balance", async function () {
      const { dao, token, admin1, admin2, recipient } = await loadFixture(
        deployTeamDAOFixture
      );

      const pairs = [
        { recipient: recipient.address, amount: ethers.parseEther("1000") },
      ];

      await dao.connect(admin1).proposeBatch(await token.getAddress(), pairs);

      await expect(
        dao.connect(admin2).approveAndExecute(0)
      ).to.be.revertedWithCustomError(token, "ERC20InsufficientBalance");
    });
  });
});
