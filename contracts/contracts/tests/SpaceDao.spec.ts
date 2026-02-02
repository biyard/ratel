import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("SpaceDAO", function () {
  async function deploySpaceDaoFixture() {
    const [deployer, admin1, admin2, admin3, admin4, user] =
      await ethers.getSigners();
    const admins = [admin1.address, admin2.address, admin3.address];

    const MockToken = await ethers.getContractFactory("MockToken");
    const token = await MockToken.deploy();

    const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
    const samplingConfig = { mode: 0, randomCount: 3 };
    const dao = await SpaceDAO.deploy(admins, samplingConfig);

    return {
      dao,
      token,
      admins,
      samplingConfig,
      deployer,
      admin1,
      admin2,
      admin3,
      admin4,
      user,
    };
  }

  describe("Deployment", function () {
    it("deploys with valid admins and sampling config", async function () {
      const { dao, admins, samplingConfig } = await loadFixture(
        deploySpaceDaoFixture
      );

      expect(await dao.getAdmins()).to.deep.equal(admins);
      expect(await dao.getIsAdmin(admins[0])).to.equal(true);
      expect(await dao.getIsAdmin(admins[1])).to.equal(true);
      expect(await dao.getIsAdmin(admins[2])).to.equal(true);

      const config = await dao.getSamplingConfig();
      expect(config.mode).to.equal(samplingConfig.mode);
      expect(config.randomCount).to.equal(samplingConfig.randomCount);
    });

    it("reverts when admin count is less than 3", async function () {
      const [admin1, admin2] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([admin1.address, admin2.address], {
          mode: 0,
          randomCount: 1,
        })
      ).to.be.revertedWith("SpaceDAO: at least 3 admins required");
    });

    it("reverts on invalid or duplicate admins", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([ethers.ZeroAddress, admin2.address, admin3.address], {
          mode: 0,
          randomCount: 1,
        })
      ).to.be.revertedWith("SpaceDAO: invalid admin");

      await expect(
        SpaceDAO.deploy([admin1.address, admin1.address, admin3.address], {
          mode: 0,
          randomCount: 1,
        })
      ).to.be.revertedWith("SpaceDAO: duplicate admin");
    });
  });

  describe("Sampling", function () {
    it("allows admin to sample randomly and stores results", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);

      const candidates = Array.from(
        { length: 5 },
        () => ethers.Wallet.createRandom().address
      );

      const sampled = await dao.connect(admin1).sample.staticCall(candidates);
      await dao.connect(admin1).sample(candidates);

      expect(sampled.length).to.equal(3);
      const stored = await dao.getSampledAddresses();
      expect(stored.length).to.equal(3);

      const candidateSet = new Set(candidates.map((a) => a.toLowerCase()));
      const storedSet = new Set(stored.map((a: string) => a.toLowerCase()));
      expect(storedSet.size).to.equal(3);
      for (const addr of stored) {
        expect(candidateSet.has(addr.toLowerCase())).to.equal(true);
      }
    });

    it("caps sample size to candidate length", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);
      const candidates = Array.from(
        { length: 2 },
        () => ethers.Wallet.createRandom().address
      );

      await dao.connect(admin1).sample(candidates);
      const stored = await dao.getSampledAddresses();
      expect(stored.length).to.equal(2);
    });

    it("reverts when non-admin calls sample", async function () {
      const { dao, user } = await loadFixture(deploySpaceDaoFixture);
      const candidates = [ethers.Wallet.createRandom().address];
      await expect(dao.connect(user).sample(candidates)).to.be.revertedWith(
        "SpaceDAO: admin only"
      );
    });

    it("reverts when mode is not random", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
      const dao = await SpaceDAO.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 1, randomCount: 2 }
      );

      const candidates = Array.from(
        { length: 3 },
        () => ethers.Wallet.createRandom().address
      );
      await expect(dao.connect(admin1).sample(candidates)).to.be.revertedWith(
        "SpaceDAO: mode not supported"
      );
    });
  });

  describe("Distribution", function () {
    it("reverts for non-admins and invalid inputs", async function () {
      const { dao, user } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [ethers.Wallet.createRandom().address];

      await expect(
        dao.connect(user).distribute(ethers.ZeroAddress, recipients, 1)
      ).to.be.revertedWith("SpaceDAO: admin only");
    });

    it("reverts on invalid token, recipients, or value", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [ethers.Wallet.createRandom().address];

      await expect(
        dao.connect(admin1).distribute(ethers.ZeroAddress, recipients, 1)
      ).to.be.revertedWith("SpaceDAO: invalid token");

      await expect(
        dao
          .connect(admin1)
          .distribute(ethers.Wallet.createRandom().address, [], 1)
      ).to.be.revertedWith("SpaceDAO: empty recipients");

      await expect(
        dao
          .connect(admin1)
          .distribute(ethers.Wallet.createRandom().address, recipients, 0)
      ).to.be.revertedWith("SpaceDAO: invalid value");
    });

    it("reverts when balance is insufficient or recipient is invalid", async function () {
      const { dao, admin1, token } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [ethers.Wallet.createRandom().address];

      await dao.connect(admin1).setSamplingCount(recipients.length);
      await dao.connect(admin1).sample(recipients);

      await expect(
        dao.connect(admin1).distribute(await token.getAddress(), recipients, 1)
      ).to.be.revertedWith("SpaceDAO: insufficient balance");

      const amount = ethers.parseUnits("10", 18);
      await token.transfer(await dao.getAddress(), amount);
      await expect(
        dao
          .connect(admin1)
          .distribute(await token.getAddress(), [ethers.ZeroAddress], amount)
      ).to.be.revertedWith("SpaceDAO: invalid recipient");
    });

    it("distributes tokens to recipients", async function () {
      const { dao, admin1, token } = await loadFixture(deploySpaceDaoFixture);
      const recipients = Array.from(
        { length: 3 },
        () => ethers.Wallet.createRandom().address
      );

      await dao.connect(admin1).setSamplingCount(recipients.length);
      await dao.connect(admin1).sample(recipients);

      const value = ethers.parseUnits("5", 18);
      const total = value * BigInt(recipients.length);
      await token.transfer(await dao.getAddress(), total);

      await dao
        .connect(admin1)
        .distribute(await token.getAddress(), recipients, value);

      for (const addr of recipients) {
        expect(await token.balanceOf(addr)).to.equal(value);
      }
      expect(await token.balanceOf(await dao.getAddress())).to.equal(0);
    });
  });
});
