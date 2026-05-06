---
sidebar_position: 1
title: Attribute management
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Attribute management

This page is the hands-on guide to the `/credentials` surface — the layout you'll see, the two verification methods (KYC and offline codes), the attribute slots Ratel supports today, and the cryptographic proof underneath. For why credentials matter and where they're used, see the [Credentials overview](./).

## Where to manage attributes

```
/credentials
/<your-handle>/credentials
```

Either URL works — the page renders for the **signed-in viewer**, not the URL's handle owner. Open it from the user dropdown at the bottom of the sidebar, or paste either URL directly.

> **Privacy note.** The `/<handle>/credentials` URL is **session-scoped**. It always shows your own credentials back to you, no matter whose handle is in the URL. Visiting a friend's `/<their-handle>/credentials` shows your own credentials, not theirs. (Public credential surfaces — proof links someone else can verify — are different; see *Verify a credential externally* below.)

## What `/credentials` shows you

The page is laid out like an arena identity card, top to bottom:

### Hero — your Personal Identity card

A single **Personal Identity** card shows:

- **Issuer** — *Ratel Foundation*. (Bound to your account; no wallet required.)
- **DID (Decentralized Identifier)** — your full DID string, with a **Copy DID** button. The DID is the public identifier other people verify against.
- **Issued / Expires** — when the DID was issued and when it expires (currently 1 year from issuance).
- **Attribute count** — *N attributes verified* — how many claims are bound to this DID.
- **QR code** — a scannable code that links to your DID's public verification endpoint, for in-person attestation.

### Verification methods

Two paths to verify an attribute, with separate cards explaining each:

- <img src={useBaseUrl('/img/icons/award.svg')} width="16" alt="KYC" style={{verticalAlign: 'middle'}} /> **KYC · PortOne** — Real-name identity check via PortOne's KCB integration. Ratel receives only **age and gender** from PortOne; your birthdate, ID number, and full name **never leave the KYC provider**. Click **Start KYC** (or **Re-run KYC**) to launch the PortOne flow.
- **Code Verification** — Offline-distributed one-time codes. Institutions — universities, employers, conferences, DAOs — hand out codes in a format like `RTL-SNU-7F3A-9CB2`. Paste the code in the **Enter verification code** modal and Ratel attests the attribute without ever contacting the institution.

### Attributes

A grid below the methods showing each attribute that's been verified — **Age**, **Gender**, **University**, **Employer**, **Membership** are the slots today. Each tile shows:

- The attribute name and value (or *Not verified* if blank).
- Which method verified it (*KYC* or *Code*).
- The institution / source label when relevant (e.g. *PortOne* for KYC, the issuing institution's code for Code).

Empty attribute slots show an *Add code* / *Start KYC* call-to-action so you can fill them in directly from the grid.

If you have **no credentials yet**, the page shows an empty-state hero: *"Your DID exists, but no verifiable credentials are bound to it. Run KYC or redeem a code to unlock age-gated spaces and reward boosts."*

### Cryptographic Proof panel

A panel at the bottom of the page surfaces the *technical* trust chain so anyone — Spaces, partners, you — can audit:

- **VC format** — *W3C VC 2.0* (the W3C Verifiable Credentials standard).
- **Proof suite** — *Ed25519Signature2020* (the cryptographic signature format).
- **Subject** — your DID.
- **Issued** — timestamp.
- **Issuer key** — *Active*.
- **Integrity** — *JSON-LD hash matches*.
- **Revocation** — *Not revoked*.
- **Signed by** — *Ratel Foundation · valid for 1 year*.

An **Export VC** button at the top of the page exports your credential as a portable W3C VC document.

## Verify a credential externally

When you want to prove an attribute to someone *outside* Ratel — a Space gating membership, an external partner, a friend wanting to confirm you really are who you say — point them at the public verification endpoint encoded in your DID's QR code. They scan or paste the QR contents and get an externally verifiable answer back, without seeing the underlying data the credential is built from.

The flow is **selective disclosure** by design: a verifier asks *"is this person 19+?"* and Ratel returns yes/no — your birthdate, ID number, and full name never enter the answer.

## Privacy guarantees

Your data, your disclosure. A few principles the page enforces:

- **Selective disclosure** — A verifier sees only the attribute they asked for (e.g. *"age ≥ 19"*), not the data behind it (your actual birthdate). KYC providers see your raw ID; Ratel does not.
- **No wallet required** — The DID is bound to your Ratel account. You don't need a blockchain wallet to *have* a DID; you only need one to *claim on-chain rewards* tied to it (see [Rewards](../rewards/tokens)).
- **KMS-encrypted at rest** — Cryptographic material (the issuer's signing key, your bound DID, the W3C VC document) is KMS-encrypted on Ratel's side. Verifiable signatures travel; secrets don't.
- **Revocable** — *Not revoked* today, but the model includes revocation. If a credential needs to be invalidated (a code is leaked, an institution requests it), Ratel can mark it revoked and verifiers checking the proof get *Revoked* in their answer.

## What's *(Coming soon)*

- **Wallet-bound DIDs** — Today the DID is bound to your Ratel account. A self-sovereign wallet-bound DID flow that exports the DID itself to a wallet you fully control is on the roadmap.
- **More attribute slots** — Today the grid shows Age, Gender, University, Employer, Membership. Custom institutional attributes (volunteer hours, certification grades, contribution badges) are *(Coming soon)*.
- **Multi-issuer credentials** — At present, Ratel Foundation is the sole issuer. Letting other institutions issue VCs into the same DID — without going through Ratel as middleman — is on the roadmap.
- **Public verifier UI** — A polished web verifier page where someone with your QR code can drop it in and see the answer back, end to end, without needing a Ratel account.
