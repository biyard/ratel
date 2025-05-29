const yargs = require("yargs");
const fs = require("fs");
const path = require("path");
const Caver = require("caver-js");
const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);
require("dotenv").config();

class ContractCLI {
  constructor(contractName) {
    this.contractName = contractName;

    this.owner = process.env.CLI_CONTRACT_OWNER_ADDR;
    this.ownerKey = process.env.CLI_CONTRACT_OWNER_KEY;
    this.feePayer = process.env.CLI_FEEPAYER_ADDR;
    this.feepayerKey = process.env.CLI_FEEPAYER_KEY;
    this.endpoint = process.env.CLI_ENDPOINT;

    const caver = new Caver(this.endpoint);
    const feepayerKeyring = new caver.wallet.keyring.singleKeyring(
      this.feePayer,
      this.feepayerKey,
    );
    const ownerKeyring = new caver.wallet.keyring.singleKeyring(
      this.owner,
      this.ownerKey,
    );
    caver.wallet.add(feepayerKeyring);
    if (this.owner !== this.feePayer) {
      caver.wallet.add(ownerKeyring);
    }

    this.caver = caver;
  }

  async loadContract() {
    const { stdout, stderr } = await exec(
      `find ./artifacts -name ${this.contractName}.json`,
    );
    const file = stdout.replace(/(\r\n|\n|\r)/gm, "");
    this.data = require(file);
  }

  async getBytecode() {
    await this.loadContract();
    return this.data.bytecode;
  }

  async listMethods() {
    await this.loadContract();
    const contract = new this.caver.contract(this.data.abi, this.owner, {
      feePayer: this.feePayer,
    });
    const methods = Object.keys(contract.methods).filter((k) =>
      k.includes("("),
    );
    methods.forEach((el) => console.log(el));
  }

  async execute(contractAddress, functionName, args) {
    await this.loadContract();
    const contract = new this.caver.contract(this.data.abi, contractAddress);
    const method = contract.methods[functionName];
    let ret;

    if (args.length > 0) {
      ret = await method(...args).send({
        feePayer: this.feePayer,
        gas: 100000000,
        feeDelegation: true,
        from: this.owner,
      });
    } else {
      ret = await method().send({
        feePayer: this.feePayer,
        feeDelegation: true,
        gas: 100000000,
        from: this.owner,
      });
    }

    return ret;
  }

  async call(contractAddress, functionName, args) {
    await this.loadContract();
    const contract = new this.caver.contract(this.data.abi, contractAddress);
    const method = contract.methods[functionName];
    let ret;

    if (args?.length > 0) {
      ret = await method(...args).call({ from: this.owner });
    } else {
      ret = await method().call({ from: this.owner });
    }

    return ret;
  }

  async deploy(args) {
    await this.loadContract();
    try {
      const contract = new this.caver.contract(this.data.abi, this.owner, {
        feePayer: this.feePayer,
      });

      const result = await contract.deploy(
        {
          from: this.owner,
          gas: 100000000,
          value: 0,
          feePayer: this.feePayer,
          feeDelegation: true,
        },
        this.data.bytecode,
        ...args,
      );

      return result._address;
    } catch (e) {
      console.log("error");
      console.trace(e);
    }
  }
}

(async () => {
  const { argv } = yargs(process.argv.splice(2))
    .command(
      "deploy <contractName> [args...]",
      "deploy a contract",
      (yargs) => {
        return yargs
          .positional("contractName", {
            describe: "contract name you want to deploy",
          })
          .positional("args", {
            array: true,
            type: "string",
          });
      },
      async (argv) => {
        const cli = new ContractCLI(argv.contractName);
        const addr = await cli.deploy(argv.args);
        console.log(addr);
      },
    )

    .command(
      "methods <contractName>",
      "list methods of a contract",
      (yargs) => {
        return yargs.positional("contractName", {
          describe: "contract name you want to list functions",
        });
      },
      (argv) => {
        const cli = new ContractCLI(argv.contractName);
        cli.listMethods();
      },
    )

    .command(
      "call <contractName> <contractAddress> <function> [args...]",
      "call a function",
      (yargs) => {
        return yargs
          .positional("contractName", {
            describe: "contract name you want to call",
          })
          .positional("contractAddress", {
            type: "string",
            describe: "contract address you want to execute",
          })
          .positional("function", {
            describe: "function name you want to execute",
          })
          .positional("args", {
            array: true,
            type: "string",
          });
      },
      async (argv) => {
        const cli = new ContractCLI(argv.contractName);
        const ret = await cli.call(
          argv.contractAddress,
          argv.function,
          argv.args,
        );
        console.log(`call result: `, ret);
      },
    )

    .command(
      "execute <contractName> <contractAddress> <function> [args...]",
      "execute a function",
      (yargs) => {
        return yargs
          .positional("contractName", {
            describe: "contract name you want to call",
          })
          .positional("contractAddress", {
            type: "string",
            describe: "contract address you want to execute",
          })
          .positional("function", {
            describe: "function name you want to execute",
          })
          .positional("args", {
            array: true,
            type: "string",
          });
      },
      async (argv) => {
        const cli = new ContractCLI(argv.contractName);
        const ret = await cli.execute(
          argv.contractAddress,
          argv.function,
          argv.args,
        );

        console.log("execute result:", ret);
      },
    )

    .help("h");
})();
