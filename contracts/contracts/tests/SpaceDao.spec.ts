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
    const incentiveDistributionConfig = {
      mode: 0,
      numOfTargets: 3,
      rankingBps: 0,
    };
    const dao = await SpaceDAO.deploy(admins, incentiveDistributionConfig);

    return {
      dao,
      token,
      admins,
      incentiveDistributionConfig,
      deployer,
      admin1,
      admin2,
      admin3,
      admin4,
      user,
    };
  }

  describe("Deployment", function () {
    it("deploys with valid admins and incentive distribution config", async function () {
      const { dao, admins, incentiveDistributionConfig } = await loadFixture(
        deploySpaceDaoFixture
      );

      expect(await dao.getAdmins()).to.deep.equal(admins);
      expect(await dao.getIsAdmin(admins[0])).to.equal(true);
      expect(await dao.getIsAdmin(admins[1])).to.equal(true);
      expect(await dao.getIsAdmin(admins[2])).to.equal(true);

      const config = await dao.getIncentiveDistributionConfig();
      expect(config.mode).to.equal(incentiveDistributionConfig.mode);
      expect(config.numOfTargets).to.equal(
        incentiveDistributionConfig.numOfTargets
      );
      expect(config.rankingBps).to.equal(incentiveDistributionConfig.rankingBps);
    });

    it("reverts when admin count is zero", async function () {
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([], {
          mode: 0,
          numOfTargets: 1,
          rankingBps: 0,
        })
      ).to.be.revertedWith("SpaceDAO: empty admins");
    });

    it("reverts on invalid or duplicate admins", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");

      await expect(
        SpaceDAO.deploy([ethers.ZeroAddress, admin2.address, admin3.address], {
          mode: 0,
          numOfTargets: 1,
          rankingBps: 0,
        })
      ).to.be.revertedWith("SpaceDAO: invalid admin");

      await expect(
        SpaceDAO.deploy([admin1.address, admin1.address, admin3.address], {
          mode: 0,
          numOfTargets: 1,
          rankingBps: 0,
        })
      ).to.be.revertedWith("SpaceDAO: duplicate admin");
    });
  });

  describe("Admin management", function () {
    it("allows admin to update ranking ratio", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);

      await dao.connect(admin1).setIncentiveRankingBps(2500);
      const config = await dao.getIncentiveDistributionConfig();
      expect(config.rankingBps).to.equal(2500);
    });

    it("reverts when non-admin updates ranking ratio", async function () {
      const { dao, user } = await loadFixture(deploySpaceDaoFixture);

      await expect(
        dao.connect(user).setIncentiveRankingBps(1000)
      ).to.be.revertedWith("SpaceDAO: admin only");
    });

    it("reverts on invalid ranking ratio", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);

      await expect(
        dao.connect(admin1).setIncentiveRankingBps(10001)
      ).to.be.revertedWith("SpaceDAO: invalid ranking bps");
    });
  });

  describe("IncentiveDistribution selection", function () {
    it("maps weighted random ranges to expected indexes", async function () {
      const SpaceDaoBitHarness = await ethers.getContractFactory(
        "SpaceDaoBitHarness"
      );
      const harness = await SpaceDaoBitHarness.deploy();

      const scores = [50, 30, 20, 0, 10];
      const excluded = [false, false, false, false, false];

      expect(await harness.findIndex(scores, excluded, 0)).to.equal(0);
      expect(await harness.findIndex(scores, excluded, 49)).to.equal(0);
      expect(await harness.findIndex(scores, excluded, 50)).to.equal(1);
      expect(await harness.findIndex(scores, excluded, 79)).to.equal(1);
      expect(await harness.findIndex(scores, excluded, 80)).to.equal(2);
      expect(await harness.findIndex(scores, excluded, 100)).to.equal(4);
    });

    it("allows admin to select recipients randomly and stores results", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);

      const candidates = Array.from(
        { length: 5 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      const selected = await dao
        .connect(admin1)
        .selectIncentiveRecipients.staticCall(candidates, scores);
      await dao
        .connect(admin1)
        .selectIncentiveRecipients(candidates, scores, { gasLimit: 16_000_000 });

      expect(selected.length).to.equal(3);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(3);

      const candidateSet = new Set(candidates.map((a) => a.toLowerCase()));
      const storedSet = new Set(stored.map((a: string) => a.toLowerCase()));
      expect(storedSet.size).to.equal(3);
      for (const addr of stored) {
        expect(candidateSet.has(addr.toLowerCase())).to.equal(true);
      }
    });

    it("caps selection size to candidate length", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);
      const candidates = Array.from(
        { length: 2 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(2);
    });

    it("reverts when non-admin calls select", async function () {
      const { dao, user } = await loadFixture(deploySpaceDaoFixture);
      const candidates = [ethers.Wallet.createRandom().address];
      const scores = [1n];
      await expect(
        dao.connect(user).selectIncentiveRecipients(candidates, scores)
      ).to.be.revertedWith("SpaceDAO: admin only");
    });

    it("selects top-ranked addresses in ranking mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
      const dao = await SpaceDAO.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 1, numOfTargets: 2, rankingBps: 10000 }
      );

      const candidates = [
        ethers.Wallet.createRandom().address,
        ethers.Wallet.createRandom().address,
        ethers.Wallet.createRandom().address,
      ];
      const scores = [1n, 5n, 3n];

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(2);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.has(candidates[1].toLowerCase())).to.equal(true);
      expect(storedSet.has(candidates[2].toLowerCase())).to.equal(true);
    });

    it("handles 100 candidates", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);
      await dao.connect(admin1).setIncentiveRecipientCount(10);

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(10);
    });

    it("handles 100 candidates in mixed mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
      const dao = await SpaceDAO.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 20, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(20);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.size).to.equal(20);
    });

    it("selects 100 recipients in mixed mode with 100 candidates", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
      const dao = await SpaceDAO.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 100, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      expect(stored.length).to.equal(100);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.size).to.equal(100);
    });

    it("selects 100 recipients from 800 candidates in mixed mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceDAO = await ethers.getContractFactory("SpaceDAO");
      const dao = await SpaceDAO.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 100, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 800 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await dao.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await dao.getIncentiveRecipients();
      console.log("stored address:", stored);
      expect(stored.length).to.equal(100);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.size).to.equal(100);
    });
  });

  describe("Incentive Claim", function () {
    it("reverts on invalid token", async function () {
      const { dao, admin1 } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [ethers.Wallet.createRandom().address];
      const scores = [1n];

      await dao.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await dao.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        dao.connect(admin1).claimIncentive(ethers.ZeroAddress)
      ).to.be.revertedWith("SpaceDAO: invalid token");
    });

    it("reverts when caller is not selected or already claimed", async function () {
      const { dao, admin1, token, user } = await loadFixture(
        deploySpaceDaoFixture
      );
      const recipients = [admin1.address];
      const scores = [1n];

      await dao.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await dao.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        dao.connect(user).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceDAO: not selected");

      const amount = ethers.parseUnits("5", 18);
      await token.transfer(await dao.getAddress(), amount);
      await dao.connect(admin1).claimIncentive(await token.getAddress());

      await expect(
        dao.connect(admin1).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceDAO: incentive finished");
    });

    it("reverts when balance is zero", async function () {
      const { dao, admin1, token } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [admin1.address];
      const scores = [1n];

      await dao.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await dao.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        dao.connect(admin1).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceDAO: invalid value");
    });

    it("allows selected recipient to claim once", async function () {
      const { dao, admin1, token } = await loadFixture(deploySpaceDaoFixture);
      const recipients = [admin1.address];
      const scores = [1n];

      await dao.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await dao.connect(admin1).selectIncentiveRecipients(recipients, scores);

      const value = ethers.parseUnits("5", 18);
      await token.transfer(await dao.getAddress(), value);

      await dao.connect(admin1).claimIncentive(await token.getAddress());

      expect(await token.balanceOf(admin1.address)).to.equal(value);
      expect(await token.balanceOf(await dao.getAddress())).to.equal(0);
    });
  });
});
