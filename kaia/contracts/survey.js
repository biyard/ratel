const Caver = require("caver-js");
require("dotenv").config();

function must(v, k) {
  if (!v || v === "") throw new Error(`Missing env: ${k}`);
  return v;
}

async function deploy(caver, artifact, args, { from, gas, label }) {
  const c = new caver.contract(artifact.abi);
  const deployed = await c
    .deploy({ data: artifact.bytecode, arguments: args })
    .send({
      from,
      gas,
    });
  const addr = deployed.options.address;
  console.log(`[DEPLOY] ${label}: ${addr}`);
  return addr;
}

async function sendTx(method, { from, gas, label }) {
  const tx = await method.send({
    from,
    gas,
  });
  const hash = tx.transactionHash || tx;
  console.log(`[TX] ${label}: ${hash}`);
  return tx;
}

function pickEvent(receipt, eventName) {
  if (!receipt || !receipt.events) return null;
  const e = receipt.events[eventName];
  if (!e) return null;
  return Array.isArray(e) ? e[0] : e;
}

(async () => {
  const owner = must(
    process.env.CLI_CONTRACT_OWNER_ADDR,
    "CLI_CONTRACT_OWNER_ADDR"
  );
  const ownerKey = must(
    process.env.CLI_CONTRACT_OWNER_KEY,
    "CLI_CONTRACT_OWNER_KEY"
  );

  const endpoint = must(process.env.CLI_ENDPOINT, "CLI_ENDPOINT");

  const caver = new Caver(endpoint);
  caver.wallet.add(caver.wallet.keyring.create(owner, ownerKey));

  const DaoRegistryStateV1 = require("./artifacts/contracts/dao-registry/DaoRegistryStateV1.sol/DaoRegistryStateV1.json");
  const DaoRegistry = require("./artifacts/contracts/dao-registry/DaoRegistry.sol/DaoRegistry.json");

  const SurveyDaoStateV1 = require("./artifacts/contracts/survey-dao/SurveyDaoStateV1.sol/SurveyDaoStateV1.json");
  const SurveyDao = require("./artifacts/contracts/survey-dao/SurveyDao.sol/SurveyDao.json");

  const registryName = "Ratel-DAO-Registry";
  const registryInitialOperator = owner;

  const surveyName = "공론조사_2차 토론방(전원)";
  const surveyDaoManager = owner;
  const surveyOperator = owner;

  const templateInput = {
    topic: "공론조사_2차 토론방(전원)",
    purpose:
      "1차 토론 요약(파일 탭)을 바탕으로 2차 토론을 진행하고, 최종 설문 참여까지 완료하기 위함",
    background:
      "지금부터는 남녀 성별 구분 없이 다같이 토론을 시작합니다. '파일' 탭에는 1차 토론 내용이 정리되어 있으며, AI로 요약한 내용과 원본도 함께 업로드되어 있습니다. 토론은 반드시 확인된 정보만 기반으로 작성해야 하며, 모든 과정(사전 설문조사-1차 토론-2차 토론-사후 설문조사)에 참여해야 사례비 지급이 가능합니다.",
    responseMethod:
      "파일 탭 요약/원본 확인 → 게시판에서 2차 토론 참여(11/27 09:00~11/28 20:00 KST) → 11/28 20:00~23:00 설문 탭 최종 설문 참여",
    configVoteDurationSecs: 35 * 60 * 60,
  };

  console.log("\n=== 1) Deploy DaoRegistryStateV1 ===");
  const registryStateAddr = await deploy(caver, DaoRegistryStateV1, [owner], {
    from: owner,
    gas: 25_000_000,
    label: "DaoRegistryStateV1",
  });

  const registryState = new caver.contract(
    DaoRegistryStateV1.abi,
    registryStateAddr
  );

  console.log("\n=== 2) Bootstrap: registryState.allowPublicWrite(true) ===");
  await sendTx(registryState.methods.allowPublicWrite(true), {
    from: owner,
    gas: 3_000_000,
    label: "DaoRegistryStateV1.allowPublicWrite(true)",
  });

  console.log("\n=== 3) Deploy DaoRegistry ===");
  const registryAddr = await deploy(
    caver,
    DaoRegistry,
    [registryName, registryStateAddr, registryInitialOperator],
    { from: owner, gas: 35_000_000, label: "DaoRegistry" }
  );

  console.log("\n=== 4) Lock back: registryState.allowPublicWrite(false) ===");
  await sendTx(registryState.methods.allowPublicWrite(false), {
    from: owner,
    gas: 3_000_000,
    label: "DaoRegistryStateV1.allowPublicWrite(false)",
  });

  const registry = new caver.contract(DaoRegistry.abi, registryAddr);

  console.log("\n=== 5) Deploy SurveyDaoStateV1 (offchain deploy) ===");
  const surveyStateAddr = await deploy(
    caver,
    SurveyDaoStateV1,
    [surveyDaoManager, surveyOperator],
    { from: owner, gas: 25_000_000, label: "SurveyDaoStateV1" }
  );

  console.log("\n=== 6) Deploy SurveyDao (offchain deploy) ===");
  const surveyDaoAddr = await deploy(caver, SurveyDao, [surveyStateAddr], {
    from: owner,
    gas: 25_000_000,
    label: "SurveyDao",
  });

  const surveyState = new caver.contract(SurveyDaoStateV1.abi, surveyStateAddr);
  const surveyDao = new caver.contract(SurveyDao.abi, surveyDaoAddr);

  console.log("\n=== 7) SurveyState.addOperator(SurveyDao) ===");
  await sendTx(surveyState.methods.addOperator(surveyDaoAddr), {
    from: owner,
    gas: 3_000_000,
    label: "SurveyDaoStateV1.addOperator(surveyDao)",
  });

  console.log(
    "check daoManager:",
    await surveyState.methods.daoManager().call()
  );
  console.log(
    "check isOperator(surveyDao):",
    await surveyState.methods.listOperators().call(),
    surveyDaoAddr
  );

  console.log(
    "\n=== 8) Registry.registerSurveyDao(name, operator, dao, state) ==="
  );
  const regReceipt = await sendTx(
    registry.methods.registerSurveyDao(
      surveyName,
      surveyOperator,
      surveyDaoAddr,
      surveyStateAddr
    ),
    {
      from: owner,
      gas: 6_000_000,
      label: "DaoRegistry.registerSurveyDao",
    }
  );

  let daoId = null;
  const ev = pickEvent(regReceipt, "SurveyDaoCreated");
  if (ev && ev.returnValues && ev.returnValues.daoId != null) {
    daoId = ev.returnValues.daoId;
  }

  console.log("\n=== 9) SurveyDao.createTemplate(...) ===");
  const ctReceipt = await sendTx(
    surveyDao.methods.createTemplate(
      templateInput.topic,
      templateInput.purpose,
      templateInput.background,
      templateInput.responseMethod,
      templateInput.configVoteDurationSecs
    ),
    { from: owner, gas: 6_000_000, label: "SurveyDao.createTemplate" }
  );

  let templateId = null;
  const ev2 = pickEvent(ctReceipt, "TemplateCreated");
  if (ev2 && ev2.returnValues && ev2.returnValues.templateId != null) {
    templateId = ev2.returnValues.templateId;
  }

  console.log("\n=== RESULT ===");
  console.log("RegistryState :", registryStateAddr);
  console.log("Registry      :", registryAddr);
  console.log("SurveyState   :", surveyStateAddr);
  console.log("SurveyDao     :", surveyDaoAddr);
  console.log("daoId         :", daoId);
  console.log("templateId    :", templateId);

  const count = await surveyState.methods.templateCount().call({ from: owner });
  console.log("templateCount :", count);
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
