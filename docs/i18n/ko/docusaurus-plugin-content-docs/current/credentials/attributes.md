---
sidebar_position: 1
title: 속성 관리
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 속성 관리

이 페이지는 `/credentials` 화면을 직접 다루는 가이드입니다 — 페이지 구성, 두 가지 인증 수단(KYC 와 오프라인 코드), Ratel 이 오늘 지원하는 속성 슬롯, 그 아래의 암호학적 증명. 자격증명이 왜 중요한지, 어디에 쓰이는지에 대한 상위 개요는 [자격증명 개요](./) 를 참고하세요.

## 속성을 관리하는 위치

```
/credentials
/<your-handle>/credentials
```

두 URL 모두 동작합니다 — 페이지는 핸들 소유자가 아니라 **로그인한 방문자** 의 자격증명을 렌더링합니다. 사이드바 하단의 사용자 드롭다운에서 열거나, 두 URL 중 하나를 직접 붙여넣어 접근하세요.

> **프라이버시 안내.** `/<handle>/credentials` URL 은 **세션 범위** 입니다. URL 의 핸들이 누구든 항상 본인의 자격증명을 본인에게 보여줍니다. 친구의 `/<their-handle>/credentials` 를 방문해도 그 친구의 자격증명이 아니라 본인의 자격증명이 보여요. (다른 사람이 검증할 수 있는 *공개* 자격증명 노출은 다른 흐름입니다 — 아래 *외부에서 자격증명 검증하기* 참고.)

## `/credentials` 가 보여주는 것

페이지는 위에서 아래로 아레나 신원 카드처럼 구성됩니다.

### 히어로 — Personal Identity 카드

단일 **Personal Identity (개인 신원)** 카드에 다음이 표시됩니다.

- **발급자(Issuer)** — *Ratel Foundation*. (계정에 묶여 있어 지갑이 필요 없습니다.)
- **DID (분산 식별자)** — 전체 DID 문자열, **DID 복사** 버튼 포함. DID 는 다른 사람이 검증의 기준으로 삼는 공개 식별자입니다.
- **발급 / 만료(Issued / Expires)** — DID 가 언제 발급됐고 언제 만료되는지 (현재 발급 후 1 년).
- **속성 카운트(Attribute count)** — *N attributes verified* — 이 DID 에 묶인 클레임 수.
- **QR 코드** — DID 의 공개 검증 엔드포인트로 연결되는 스캔 가능 코드, 대면 인증에 유용합니다.

### 인증 수단

속성을 인증하는 두 경로, 각각 별도 카드로 설명됩니다.

- <img src={useBaseUrl('/img/icons/award.svg')} width="16" alt="KYC" style={{verticalAlign: 'middle'}} /> **KYC · PortOne** — PortOne 의 KCB 연동을 통한 실명 인증. PortOne 으로부터 Ratel 은 **나이와 성별** 만 받습니다 — 생년월일, 주민등록번호, 실명은 **KYC 제공자를 절대 떠나지 않습니다**. **Start KYC** (또는 **Re-run KYC**) 를 누르면 PortOne 흐름이 시작돼요.
- **코드 인증(Code Verification)** — 오프라인으로 배포되는 일회용 코드. 대학, 직장, 컨퍼런스, DAO 등 기관이 `RTL-SNU-7F3A-9CB2` 같은 형식의 코드를 오프라인으로 나눠 줍니다. **Enter verification code (인증 코드 입력)** 모달에 코드를 붙여 넣으면 Ratel 이 기관에 연락하지 않고 그 속성을 증명합니다.

### 속성

인증 수단 아래의 그리드는 인증된 각 속성을 보여줍니다 — 오늘 시점의 슬롯은 **나이(Age)**, **성별(Gender)**, **대학교(University)**, **직장(Employer)**, **멤버십(Membership)** 입니다. 각 타일에는:

- 속성 이름과 값 (비어 있다면 *Not verified*).
- 어떤 수단으로 인증되었는지 (*KYC* 또는 *Code*).
- 관련 기관 / 소스 라벨 (예: KYC 의 *PortOne*, 코드의 발급 기관 코드).

비어 있는 속성 슬롯에는 *Add code* / *Start KYC* 가 노출되어 그리드에서 바로 채울 수 있습니다.

**아직 자격증명이 없다면** 페이지가 빈 상태 히어로를 보여줍니다 — *"Your DID exists, but no verifiable credentials are bound to it. Run KYC or redeem a code to unlock age-gated spaces and reward boosts."*

### 암호학적 증명(Cryptographic Proof) 패널

페이지 하단의 패널이 *기술적* 신뢰 체인을 노출하여 누구든 — 스페이스, 파트너, 본인 — 감사할 수 있게 합니다.

- **VC 형식(VC format)** — *W3C VC 2.0* (W3C 검증 가능 자격증명 표준).
- **서명 방식(Proof suite)** — *Ed25519Signature2020* (암호 서명 형식).
- **대상(Subject)** — 본인의 DID.
- **발급(Issued)** — 타임스탬프.
- **발급자 키(Issuer key)** — *Active* (활성).
- **무결성(Integrity)** — *JSON-LD hash matches* (해시 일치).
- **취소(Revocation)** — *Not revoked* (취소되지 않음).
- **서명자(Signed by)** — *Ratel Foundation · valid for 1 year*.

페이지 상단의 **Export VC (VC 내보내기)** 버튼은 본인의 자격증명을 휴대 가능한 W3C VC 문서로 내보냅니다.

## 외부에서 자격증명 검증하기

Ratel *외부* 의 누군가에게 속성을 증명하고 싶을 때 — 멤버십을 게이팅하는 스페이스, 외부 파트너, 본인이 진짜 본인임을 확인하고 싶은 친구 — DID 의 QR 코드에 인코딩된 공개 검증 엔드포인트를 안내하세요. 스캔하거나 QR 내용을 붙여넣으면, 자격증명의 토대가 되는 데이터 자체는 보지 않은 채로 외부에서 검증 가능한 응답을 받습니다.

흐름은 설계상 **선택적 공개(selective disclosure)** 입니다 — 검증자가 *"이 사람 19 세 이상인가?"* 라고 물으면 Ratel 이 yes/no 만 응답합니다 — 답에는 생년월일, 주민등록번호, 실명이 들어가지 않아요.

## 프라이버시 보장

내 데이터, 내 공개 결정. 페이지가 보장하는 원칙은 다음과 같습니다.

- **선택적 공개** — 검증자는 자기가 물은 속성 (예: *"19 세 이상"*) 만 보고, 그 뒤의 데이터 (실제 생년월일) 는 보지 않습니다. KYC 제공자는 원본 신원을 보지만, Ratel 은 보지 않습니다.
- **지갑 불필요** — DID 는 Ratel 계정에 묶여 있습니다. DID 를 *가지는* 데는 블록체인 지갑이 필요 없고, 거기에 연결된 *온체인 보상을 청구하는* 데에만 지갑이 필요합니다 ([토큰](../rewards/tokens) 참고).
- **저장 시 KMS 암호화** — 암호학적 자료 (발급자의 서명 키, 본인에게 묶인 DID, W3C VC 문서) 는 Ratel 측에서 KMS 암호화됩니다. 검증 가능한 서명은 외부로 이동하지만, 비밀은 이동하지 않아요.
- **취소 가능** — 오늘은 *Not revoked* 이지만, 모델 자체에 취소가 포함되어 있습니다. 자격증명이 무효화되어야 한다면 (코드 유출, 기관 요청 등) Ratel 이 *Revoked* 로 표시할 수 있고, 증명을 검증하는 사람은 *Revoked* 응답을 받게 됩니다.

## *(예정)* 항목

- **지갑 바인딩 DID** — 오늘은 DID 가 Ratel 계정에 묶여 있습니다. DID 자체를 본인이 완전히 통제하는 지갑으로 내보내는 셀프 소버린 흐름은 로드맵에 있어요.
- **더 많은 속성 슬롯** — 오늘 그리드는 나이, 성별, 대학교, 직장, 멤버십을 보여줍니다. 커스텀 기관 속성 (자원봉사 시간, 인증 등급, 기여 뱃지) 은 *(예정)* 입니다.
- **다중 발급자 자격증명** — 현재는 Ratel Foundation 이 유일한 발급자입니다. 다른 기관이 Ratel 을 거치지 않고 같은 DID 에 VC 를 발급하는 모델은 로드맵에 있습니다.
- **공개 검증 UI** — 본인의 QR 코드를 받은 사람이 Ratel 계정 없이도 끝까지 검증할 수 있는 정돈된 웹 검증 페이지.
