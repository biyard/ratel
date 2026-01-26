import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("SpaceDAO", function () {
  async function deploySpaceDaoFixture() {
    const [deployer, admin1, admin2, admin3, admin4, user, recipient1, recipient2] =
      await ethers.getSigners();
    const admins = [admin1.address, admin2.address, admin3.address];

    const MockToken = await ethers.getContractFactory("MockToken");
    const token = await MockToken.deploy();

    const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
    const withdrawalAmount = ethers.parseUnits("100", 18);
    const dao = await SpaceDAO.deploy(admins, await token.getAddress(), withdrawalAmount);

    return {
      dao,
      token,
      admins,
      withdrawalAmount,
      deployer,
      admin1,
      admin2,
      admin3,
      admin4,
      user,
      recipient1,
      recipient2,
    };
  }

  describe("Deployment", function () {
    it("deploys with valid admins and parameters", async function () {
      const { dao, token, admins, withdrawalAmount } = await loadFixture(
        deploySpaceDaoFixture
      );

      expect(await dao.getUsdt()).to.equal(await token.getAddress());
      expect(await dao.getWithdrawalAmount()).to.equal(withdrawalAmount);
      expect(await dao.getAdmins()).to.deep.equal(admins);
      expect(await dao.getIsAdmin(admins[0])).to.equal(true);
      expect(await dao.getIsAdmin(admins[1])).to.equal(true);
      expect(await dao.getIsAdmin(admins[2])).to.equal(true);
    });

    it("reverts when admin count is less than 3", async function () {
      const [admin1, admin2, user] = await ethers.getSigners();
      const MockToken = await ethers.getContractFactory("MockToken");
      const token = await MockToken.deploy();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([admin1.address, admin2.address], await token.getAddress(), 1)
      ).to.be.revertedWith("SpaceDAO: at least 3 admins required");

      await expect(
        SpaceDAO.deploy([admin1.address, admin2.address, user.address], ethers.ZeroAddress, 1)
      ).to.be.revertedWith("SpaceDAO: invalid token address");

      await expect(
        SpaceDAO.deploy([admin1.address, admin2.address, user.address], await token.getAddress(), 0)
      ).to.be.revertedWith("SpaceDAO: invalid withdrawal amount");
    });

    it("reverts on invalid or duplicate admins", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const MockToken = await ethers.getContractFactory("MockToken");
      const token = await MockToken.deploy();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([ethers.ZeroAddress, admin2.address, admin3.address], await token.getAddress(), 1)
      ).to.be.revertedWith("SpaceDAO: invalid admin");

      await expect(
        SpaceDAO.deploy([admin1.address, admin1.address, admin3.address], await token.getAddress(), 1)
      ).to.be.revertedWith("SpaceDAO: duplicate admin");
    });
  });

  describe("Deposit & Balance", function () {
    it("allows deposit with approval and tracks balance", async function () {
      const { dao, token, deployer } = await loadFixture(deploySpaceDaoFixture);
      const amount = ethers.parseUnits("250", 18);

      await expect(dao.deposit(0)).to.be.revertedWith("SpaceDAO: amount is zero");

      await token.connect(deployer).approve(await dao.getAddress(), amount);
      await dao.deposit(amount);

      expect(await dao.getBalance()).to.equal(amount);
    });
  });

  describe("Withdraw distribution", function () {
    it("reverts for non-admins and invalid inputs", async function () {
      const { dao, admin1, user, recipient1 } = await loadFixture(
        deploySpaceDaoFixture
      );

      await expect(
        dao.connect(user).distributeWithdrawal([recipient1.address])
      ).to.be.revertedWith("SpaceDAO: admin only");

      await expect(
        dao.connect(admin1).distributeWithdrawal([])
      ).to.be.revertedWith("SpaceDAO: empty recipients");
    });

    it("reverts when balance is insufficient or recipient is invalid", async function () {
      const { dao, token, deployer, admin1, recipient1 } = await loadFixture(
        deploySpaceDaoFixture
      );

      await expect(
        dao.connect(admin1).distributeWithdrawal([recipient1.address])
      ).to.be.revertedWith("SpaceDAO: insufficient balance");

      const amount = (await dao.getWithdrawalAmount()) * 1n;
      await token.connect(deployer).approve(await dao.getAddress(), amount);
      await dao.connect(deployer).deposit(amount);

      await expect(
        dao.connect(admin1).distributeWithdrawal([ethers.ZeroAddress])
      ).to.be.revertedWith("SpaceDAO: invalid recipient");
    });

    it("distributes funds to recipients", async function () {
      const { dao, token, deployer, admin1, recipient1, recipient2, withdrawalAmount } =
        await loadFixture(deploySpaceDaoFixture);

      const total = withdrawalAmount * 2n;
      await token.connect(deployer).approve(await dao.getAddress(), total);
      await dao.connect(deployer).deposit(total);

      await dao.connect(admin1).distributeWithdrawal([
        recipient1.address,
        recipient2.address,
      ]);

      expect(await token.balanceOf(recipient1.address)).to.equal(withdrawalAmount);
      expect(await token.balanceOf(recipient2.address)).to.equal(withdrawalAmount);
      expect(await dao.getBalance()).to.equal(0);
    });
  });

  describe("Admin management", function () {
    it("allows admin to update withdrawal amount", async function () {
      const { dao, admin1, user } = await loadFixture(deploySpaceDaoFixture);

      await expect(
        dao.connect(user).setWithdrawalAmount(1)
      ).to.be.revertedWith("SpaceDAO: admin only");

      await expect(
        dao.connect(admin1).setWithdrawalAmount(0)
      ).to.be.revertedWith("SpaceDAO: invalid withdrawal amount");

      await dao.connect(admin1).setWithdrawalAmount(123);
      expect(await dao.getWithdrawalAmount()).to.equal(123);
    });

    it("allows admin to add a new admin", async function () {
      const { dao, admin1, admin4, user } = await loadFixture(
        deploySpaceDaoFixture
      );

      await expect(
        dao.connect(user).addAdmin(admin4.address)
      ).to.be.revertedWith("SpaceDAO: admin only");

      await expect(
        dao.connect(admin1).addAdmin(ethers.ZeroAddress)
      ).to.be.revertedWith("SpaceDAO: invalid admin");

      await dao.connect(admin1).addAdmin(admin4.address);
      expect(await dao.getIsAdmin(admin4.address)).to.equal(true);

      await expect(
        dao.connect(admin1).addAdmin(admin4.address)
      ).to.be.revertedWith("SpaceDAO: duplicate admin");
    });

    it("allows admin to update USDT address", async function () {
      const { dao, admin1, user } = await loadFixture(deploySpaceDaoFixture);
      const MockToken = await ethers.getContractFactory("MockToken");
      const newToken = await MockToken.deploy();

      await expect(
        dao.connect(user).setUsdtAddress(await newToken.getAddress())
      ).to.be.revertedWith("SpaceDAO: admin only");

      await expect(
        dao.connect(admin1).setUsdtAddress(ethers.ZeroAddress)
      ).to.be.revertedWith("Invalid USDT Address");

      await dao.connect(admin1).setUsdtAddress(await newToken.getAddress());
      expect(await dao.getUsdt()).to.equal(await newToken.getAddress());
    });
  });
});
