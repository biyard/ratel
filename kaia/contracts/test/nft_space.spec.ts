import { ContractInformation, TestingUtils } from "./util";
import { expect } from "chai";

const contractName = "Membership";

describe(contractName, function () {
  const util = new TestingUtils();
  const deploy = util.deploy;
  const owner = util.owner;
  const name = "Membership";
  const symbol = "Membership";
  const metadata = "https://example.com/metadata.json";

  let nftSpaceData: ContractInformation;

  it("deployment of nft space", async () => {
    await util.fund(util.feepayer, 100);
    await util.fund(util.owner, 100);
    const nftSpace = await deploy(contractName, owner, name, symbol, metadata);
    nftSpaceData = nftSpace;
    const n = await nftSpace.name();
    const s = await nftSpace.symbol();
    const m = await nftSpace.uri(1);

    expect(n).to.equal(name);
    expect(s).to.equal(symbol);
    expect(m).to.equal(metadata);
  });

  describe("name", async () => {
    it("set name - not owner", async () => {
      const tx = util.executeFeePayer(nftSpaceData, "setName", "New NFT Space");
      await expect(tx)
        .to.be.revertedWithCustomError(
          nftSpaceData,
          "OwnableUnauthorizedAccount"
        )
        .withArgs(util.feepayer);
    });

    it("set name", async () => {
      await util.executeOwner(nftSpaceData, "setName", "New NFT Space");
      const n = await nftSpaceData.name();
      expect(n).to.equal("New NFT Space");
    });
  });

  describe("symbol", async () => {
    it("set symbol - not owner", async () => {
      const tx = util.executeFeePayer(
        nftSpaceData,
        "setSymbol",
        "New NFT Space"
      );
      await expect(tx)
        .to.be.revertedWithCustomError(
          nftSpaceData,
          "OwnableUnauthorizedAccount"
        )
        .withArgs(util.feepayer);
    });

    it("set symbol", async () => {
      await util.executeOwner(nftSpaceData, "setSymbol", "New NFT Space");
      const s = await nftSpaceData.symbol();
      expect(s).to.equal("New NFT Space");
    });
  });

  describe("uri", async () => {
    it("set uri - not owner", async () => {
      const tx = util.executeFeePayer(
        nftSpaceData,
        "setURI",
        "https://example.com/metadata_2.json"
      );
      await expect(tx)
        .to.be.revertedWithCustomError(
          nftSpaceData,
          "OwnableUnauthorizedAccount"
        )
        .withArgs(util.feepayer);
    });

    it("set uri", async () => {
      await util.executeOwner(
        nftSpaceData,
        "setURI",
        "https://example.com/metadata_2.json"
      );
      const uri = await nftSpaceData.uri(1);
      expect(uri).to.equal("https://example.com/metadata_2.json");
    });
  });

  describe("token uri", async () => {
    it("set token uri - not owner", async () => {
      const tx = util.executeFeePayer(
        nftSpaceData,
        "setTokenURI",
        1,
        "https://example.com/metadata_3.json"
      );
      await expect(tx)
        .to.be.revertedWithCustomError(
          nftSpaceData,
          "OwnableUnauthorizedAccount"
        )
        .withArgs(util.feepayer);
    });

    it("set uri", async () => {
      await util.executeOwner(
        nftSpaceData,
        "setTokenURI",
        1,
        "https://example.com/metadata_3.json"
      );
      const uri = await nftSpaceData.uri(1);
      expect(uri).to.equal("https://example.com/metadata_3.json");

      const uri2 = await nftSpaceData.uri(2);
      expect(uri2).to.equal("https://example.com/metadata_2.json");
    });
  });

  describe("mint", async () => {
    it("mint NFT", async () => {
      await util.executeOwner(nftSpaceData, "mint", 1, 1);

      const balance = await nftSpaceData.balanceOf(owner, 1);
      expect(balance).to.equal(1);

      const isHolder = await nftSpaceData.isHolder(owner);
      expect(isHolder).to.equal(true);
    });

    it("mint NFT - already minted", async () => {
      const tx = util.executeOwner(nftSpaceData, "mint", 1, 1);
      await expect(tx).to.be.revertedWith("Already Minted");
    });
  });
});
