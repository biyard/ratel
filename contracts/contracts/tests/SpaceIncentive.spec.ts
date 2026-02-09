import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("SpaceIncentive", function () {
  async function deploySpaceIncentiveFixture() {
    const [deployer, admin1, admin2, admin3, admin4, user] =
      await ethers.getSigners();
    const admins = [admin1.address, admin2.address, admin3.address];

    const MockToken = await ethers.getContractFactory("MockToken");
    const token = await MockToken.deploy();

    const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");
    const incentiveDistributionConfig = {
      mode: 0,
      numOfTargets: 3,
      rankingBps: 0,
    };
    const incentive = await SpaceIncentive.deploy(
      admins,
      incentiveDistributionConfig
    );

    return {
      incentive,
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
      const { incentive, admins, incentiveDistributionConfig } = await loadFixture(
        deploySpaceIncentiveFixture
      );

      expect(await incentive.getAdmins()).to.deep.equal(admins);
      expect(await incentive.getIsAdmin(admins[0])).to.equal(true);
      expect(await incentive.getIsAdmin(admins[1])).to.equal(true);
      expect(await incentive.getIsAdmin(admins[2])).to.equal(true);

      const config = await incentive.getIncentiveDistributionConfig();
      expect(config.mode).to.equal(incentiveDistributionConfig.mode);
      expect(config.numOfTargets).to.equal(
        incentiveDistributionConfig.numOfTargets
      );
      expect(config.rankingBps).to.equal(
        incentiveDistributionConfig.rankingBps
      );
    });

    it("reverts when admin count is zero", async function () {
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");

      await expect(
        SpaceIncentive.deploy([], {
          mode: 0,
          numOfTargets: 1,
          rankingBps: 0,
        })
      ).to.be.revertedWith("SpaceIncentive: empty admins");
    });

    it("reverts on invalid or duplicate admins", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");

      await expect(
        SpaceIncentive.deploy(
          [ethers.ZeroAddress, admin2.address, admin3.address],
          {
            mode: 0,
            numOfTargets: 1,
            rankingBps: 0,
          }
        )
      ).to.be.revertedWith("SpaceIncentive: invalid admin");

      await expect(
        SpaceIncentive.deploy(
          [admin1.address, admin1.address, admin3.address],
          {
            mode: 0,
            numOfTargets: 1,
            rankingBps: 0,
          }
        )
      ).to.be.revertedWith("SpaceIncentive: duplicate admin");
    });
  });

  describe("Admin management", function () {
    it("allows admin to update ranking ratio", async function () {
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);

      await incentive.connect(admin1).setIncentiveRankingBps(2500);
      const config = await incentive.getIncentiveDistributionConfig();
      expect(config.rankingBps).to.equal(2500);
    });

    it("reverts when non-admin updates ranking ratio", async function () {
      const { incentive, user } = await loadFixture(deploySpaceIncentiveFixture);

      await expect(
        incentive.connect(user).setIncentiveRankingBps(1000)
      ).to.be.revertedWith("SpaceIncentive: admin only");
    });

    it("reverts on invalid ranking ratio", async function () {
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);

      await expect(
        incentive.connect(admin1).setIncentiveRankingBps(10001)
      ).to.be.revertedWith("SpaceIncentive: invalid ranking bps");
    });
  });

  describe("IncentiveDistribution selection", function () {
    it("maps weighted random ranges to expected indexes", async function () {
      const SpaceIncentiveBitHarness = await ethers.getContractFactory(
        "SpaceIncentiveBitHarness"
      );
      const harness = await SpaceIncentiveBitHarness.deploy();

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
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);

      const candidates = Array.from(
        { length: 5 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      const selected = await incentive
        .connect(admin1)
        .selectIncentiveRecipients.staticCall(candidates, scores);
      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores, {
        gasLimit: 16_000_000,
      });

      expect(selected.length).to.equal(3);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(3);

      const candidateSet = new Set(candidates.map((a) => a.toLowerCase()));
      const storedSet = new Set(stored.map((a: string) => a.toLowerCase()));
      expect(storedSet.size).to.equal(3);
      for (const addr of stored) {
        expect(candidateSet.has(addr.toLowerCase())).to.equal(true);
      }
    });

    it("caps selection size to candidate length", async function () {
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);
      const candidates = Array.from(
        { length: 2 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(2);
    });

    it("reverts when non-admin calls select", async function () {
      const { incentive, user } = await loadFixture(deploySpaceIncentiveFixture);
      const candidates = [ethers.Wallet.createRandom().address];
      const scores = [1n];
      await expect(
        incentive.connect(user).selectIncentiveRecipients(candidates, scores)
      ).to.be.revertedWith("SpaceIncentive: admin only");
    });

    it("selects top-ranked addresses in ranking mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");
      const incentive = await SpaceIncentive.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 1, numOfTargets: 2, rankingBps: 10000 }
      );

      const candidates = [
        ethers.Wallet.createRandom().address,
        ethers.Wallet.createRandom().address,
        ethers.Wallet.createRandom().address,
      ];
      const scores = [1n, 5n, 3n];

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(2);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.has(candidates[1].toLowerCase())).to.equal(true);
      expect(storedSet.has(candidates[2].toLowerCase())).to.equal(true);
    });

    it("handles 100 candidates", async function () {
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);
      await incentive.connect(admin1).setIncentiveRecipientCount(10);

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map(() => 1n);

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(10);
    });

    it("handles 100 candidates in mixed mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");
      const incentive = await SpaceIncentive.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 20, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(20);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.size).to.equal(20);
    });

    it("selects 100 recipients in mixed mode with 100 candidates", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");
      const incentive = await SpaceIncentive.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 100, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 100 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
      expect(stored.length).to.equal(100);
      const storedSet = new Set(
        stored.map((addr: string) => addr.toLowerCase())
      );
      expect(storedSet.size).to.equal(100);
    });

    it("selects 100 recipients from 800 candidates in mixed mode", async function () {
      const [admin1, admin2, admin3] = await ethers.getSigners();
      const SpaceIncentive = await ethers.getContractFactory("SpaceIncentive");
      const incentive = await SpaceIncentive.deploy(
        [admin1.address, admin2.address, admin3.address],
        { mode: 2, numOfTargets: 100, rankingBps: 3000 }
      );

      const candidates = Array.from(
        { length: 800 },
        () => ethers.Wallet.createRandom().address
      );
      const scores = candidates.map((_, idx) => BigInt(idx + 1));

      await incentive.connect(admin1).selectIncentiveRecipients(candidates, scores);
      const stored = await incentive.getIncentiveRecipients();
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
      const { incentive, admin1 } = await loadFixture(deploySpaceIncentiveFixture);
      const recipients = [ethers.Wallet.createRandom().address];
      const scores = [1n];

      await incentive.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await incentive.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        incentive.connect(admin1).claimIncentive(ethers.ZeroAddress)
      ).to.be.revertedWith("SpaceIncentive: invalid token");
    });

    it("reverts when caller is not selected or already claimed", async function () {
      const { incentive, admin1, token, user } = await loadFixture(
        deploySpaceIncentiveFixture
      );
      const recipients = [admin1.address];
      const scores = [1n];

      await incentive.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await incentive.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        incentive.connect(user).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceIncentive: not selected");

      const amount = ethers.parseUnits("5", 18);
      await token.transfer(await incentive.getAddress(), amount);
      await incentive.connect(admin1).claimIncentive(await token.getAddress());

      await expect(
        incentive.connect(admin1).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceIncentive: incentive finished");
    });

    it("reverts when balance is zero", async function () {
      const { incentive, admin1, token } = await loadFixture(deploySpaceIncentiveFixture);
      const recipients = [admin1.address];
      const scores = [1n];

      await incentive.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await incentive.connect(admin1).selectIncentiveRecipients(recipients, scores);

      await expect(
        incentive.connect(admin1).claimIncentive(await token.getAddress())
      ).to.be.revertedWith("SpaceIncentive: invalid value");
    });

    it("allows selected recipient to claim once", async function () {
      const { incentive, admin1, token } = await loadFixture(deploySpaceIncentiveFixture);
      const recipients = [admin1.address];
      const scores = [1n];

      await incentive.connect(admin1).setIncentiveRecipientCount(recipients.length);
      await incentive.connect(admin1).selectIncentiveRecipients(recipients, scores);

      const value = ethers.parseUnits("5", 18);
      await token.transfer(await incentive.getAddress(), value);

      await incentive.connect(admin1).claimIncentive(await token.getAddress());

      expect(await token.balanceOf(admin1.address)).to.equal(value);
      expect(await token.balanceOf(await incentive.getAddress())).to.equal(0);
    });
  });
});
