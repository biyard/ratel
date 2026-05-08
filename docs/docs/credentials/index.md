---
sidebar_position: 1
title: Credentials
---

# Credentials

A **Credential** on Ratel is a verifiable claim about you — your age, your affiliation with an institution, a one-time code an organizer handed out at a conference — bound to your decentralized identifier (DID) and signed by Ratel Foundation. Together, your DID and the credentials attached to it form a privacy-preserving identity wallet you can selectively disclose to Spaces, Teams, and external verifiers.

You'll reach for credentials whenever proof of *something* about you needs to leave your account without the underlying data leaving with it. The four most common uses today:

- **Gating Spaces and Actions.** A host can require an attribute (e.g. *age ≥ 19*, *member of `@university.alpha`*) before letting a participant join the Space, claim a reward, or vote in a poll. Configured from the Space's [Panels](../spaces/apps#-panels) app.
- **Verifying attributes about yourself.** Run KYC once and your age and gender become verifiable claims; redeem an offline code from an institution and that membership becomes a claim. Either way, the verifier sees the answer (*"yes, 19+"*) — never the raw data.
- **Signing votes and DAO proposals.** Your DID is what cryptographically signs the participation that flows into a Team's DAO outcomes.
- **On-chain settlement.** When you claim past-cycle [Tokens](../rewards/tokens), the on-chain transaction is signed against your DID-linked wallet — your DID is the bridge between Ratel's off-chain Points ledger and the on-chain claim.

## In this chapter

| Page | What it covers |
|---|---|
| **[Attribute management](./attributes.md)** | The hands-on `/credentials` page tour — Personal Identity card, KYC + offline code verification methods, the attribute grid (Age / Gender / University / Employer / Membership), the W3C VC cryptographic proof panel, and the privacy guarantees that hold across all of it. |
