use crate::features::social::pages::credentials::*;

translate! {
    CredentialsTranslate;

    page_title: { en: "Credentials", ko: "크리덴셜" },
    back: { en: "Back", ko: "뒤로" },
    status_verified: { en: "Verified", ko: "인증됨" },
    status_pending: { en: "Pending", ko: "미인증" },
    export_vc: { en: "Export VC", ko: "VC 내보내기" },

    hero_eyebrow: { en: "Verifiable Credential", ko: "검증 가능한 자격" },
    hero_methods: { en: "KYC + Offline Code", ko: "KYC + 오프라인 코드" },
    hero_crest_title: { en: "Personal Identity", ko: "개인 신원" },
    hero_crest_sub: {
        en: "Issued by Ratel Foundation · bound to your account (no wallet required)",
        ko: "Ratel Foundation 발급 · 계정에 귀속 (지갑 불필요)",
    },
    hero_did_label: { en: "Decentralized Identifier", ko: "탈중앙 식별자" },
    hero_issued: { en: "Issued", ko: "발급" },
    hero_expires: { en: "Expires", ko: "만료" },
    hero_verified_count: { en: "attributes verified", ko: "속성 인증됨" },
    hero_copy_did: { en: "Copy DID", ko: "DID 복사" },
    hero_qr_hint: { en: "Scan to verify credential online", ko: "스캔해서 온라인 인증" },

    stat_attributes: { en: "Attributes", ko: "속성" },
    stat_attributes_sub: { en: "verified", ko: "인증됨" },
    stat_methods: { en: "Methods", ko: "인증 수단" },
    stat_methods_sub: { en: "KYC & Code", ko: "KYC & 코드" },
    stat_last_verified: { en: "Last verified", ko: "마지막 인증" },
    stat_never: { en: "never", ko: "없음" },

    methods_title: { en: "Verification Methods", ko: "인증 수단" },
    methods_hint: {
        en: "Two paths · pick whichever fits the attribute",
        ko: "두 가지 방식 · 속성에 맞게 선택",
    },

    kyc_title: { en: "KYC · PortOne", ko: "KYC · PortOne" },
    kyc_sub_prefix: { en: "Identity ·", ko: "신원 ·" },
    kyc_sub_highlight: { en: "Age & Gender", ko: "나이 & 성별" },
    kyc_desc: {
        en: "Real-name identity check via PortOne (KCB). Ratel receives only age & gender — your birthdate, ID number, and full name never leave the KYC provider.",
        ko: "PortOne(KCB) 실명 인증. 생년월일/주민번호/이름은 인증기관 밖으로 나가지 않고, 나이·성별만 Ratel로 전달됩니다.",
    },
    kyc_rerun: { en: "Re-run KYC", ko: "KYC 재실행" },
    kyc_run: { en: "Start KYC", ko: "KYC 시작" },

    code_title: { en: "Code Verification", ko: "코드 인증" },
    code_sub_prefix: { en: "Offline-distributed ·", ko: "오프라인 배포 ·" },
    code_sub_highlight: { en: "University, Membership…", ko: "대학·멤버십…" },
    code_desc: {
        en: "Institutions — universities, employers, conferences, DAOs — hand out one-time codes offline. Paste the code and Ratel attests the attribute without ever contacting the institution.",
        ko: "대학·직장·컨퍼런스·DAO 등이 오프라인으로 배포하는 1회용 코드를 입력하면 Ratel이 속성을 증명합니다.",
    },
    code_hint: {
        en: "Got a code that looks like RTL-SNU-7F3A-9CB2? Enter it to verify.",
        ko: "RTL-SNU-7F3A-9CB2 형식의 코드가 있다면 입력해 주세요.",
    },
    code_cta: { en: "Enter verification code", ko: "인증 코드 입력" },

    attrs_title: { en: "My Attributes", ko: "내 속성" },
    attrs_hint: {
        en: "Selectively disclosed · zero-knowledge",
        ko: "선택적 공개 · 영지식",
    },

    attr_age: { en: "Age", ko: "나이" },
    attr_age_sub: { en: "years", ko: "세" },
    attr_gender: { en: "Gender", ko: "성별" },
    attr_male: { en: "Male", ko: "남성" },
    attr_female: { en: "Female", ko: "여성" },
    attr_university: { en: "University", ko: "대학교" },
    attr_employer: { en: "Employer", ko: "직장" },
    attr_membership: { en: "Membership", ko: "멤버십" },

    method_kyc: { en: "KYC", ko: "KYC" },
    method_code: { en: "Code", ko: "코드" },
    not_verified: { en: "Not verified", ko: "미인증" },
    no_codes_redeemed: { en: "No community codes redeemed", ko: "사용한 커뮤니티 코드 없음" },
    add_code_generic: { en: "Enter a code", ko: "코드 입력" },
    add_code_employer: { en: "Enter code from HR", ko: "HR 코드 입력" },
    add_code_membership: { en: "Redeem a code", ko: "코드 입력" },
    meta_portone: { en: "PortOne", ko: "PortOne" },
    meta_code_label: { en: "Code", ko: "코드" },

    proof_title: { en: "Cryptographic Proof", ko: "암호학적 증명" },
    proof_hint: {
        en: "Signature valid · issuer key active",
        ko: "서명 유효 · 발급자 키 활성",
    },
    proof_meta: { en: "Proof Metadata", ko: "증명 메타데이터" },
    proof_format: { en: "VC format", ko: "VC 형식" },
    proof_format_value: { en: "W3C VC 2.0", ko: "W3C VC 2.0" },
    proof_suite: { en: "Proof suite", ko: "서명 방식" },
    proof_suite_value: { en: "Ed25519Signature2020", ko: "Ed25519Signature2020" },
    proof_subject: { en: "Subject", ko: "대상" },
    proof_issued: { en: "Issued", ko: "발급" },
    proof_sig_title: { en: "Signature Status", ko: "서명 상태" },
    proof_issuer_key: { en: "Issuer key", ko: "발급자 키" },
    proof_issuer_key_value: { en: "Active", ko: "활성" },
    proof_integrity: { en: "Integrity", ko: "무결성" },
    proof_integrity_value: { en: "JSON-LD hash matches", ko: "JSON-LD 해시 일치" },
    proof_revocation: { en: "Revocation", ko: "취소" },
    proof_revocation_value: { en: "Not revoked", ko: "취소되지 않음" },
    proof_signed_by: { en: "Signed by", ko: "서명자" },
    proof_signed_by_suffix: {
        en: "· Ratel Foundation · valid for 1 year.",
        ko: "· Ratel Foundation · 1년간 유효.",
    },

    privacy_title: { en: "Your data, your disclosure", ko: "내 데이터, 내 공개 결정" },
    privacy_desc_prefix: { en: "Your DID is", ko: "당신의 DID는" },
    privacy_desc_suffix: {
        en: "— no wallet required. Attributes are bound to your account; when a Space or verifier requests a credential, Ratel asks for your explicit consent and shares only the minimum — e.g. 'age ≥ 19' — never your birthdate or ID number.",
        ko: "— 지갑이 필요하지 않습니다. 속성은 계정에 귀속되며, 스페이스나 검증자가 요청하면 Ratel이 명시적 동의를 받아 최소 정보(예: '만 19세 이상')만 공유합니다.",
    },

    empty_title: { en: "No credentials yet", ko: "아직 크리덴셜이 없습니다" },
    empty_desc: {
        en: "Your DID exists, but no verifiable credentials are bound to it. Run KYC or redeem a code to unlock age-gated spaces and reward boosts.",
        ko: "DID는 있지만 아직 어떤 크리덴셜도 연결되지 않았습니다. KYC 또는 코드 인증으로 연령 제한 스페이스·리워드 가산을 해제하세요.",
    },
    empty_cta: { en: "Verify your first attribute", ko: "첫 속성 인증하기" },

    verification_error: { en: "Verification failed", ko: "인증에 실패했습니다" },
    modal_code_title: { en: "Enter Code", ko: "코드 입력" },
    modal_code_placeholder: { en: "Enter verification code", ko: "인증 코드를 입력하세요" },
    modal_cancel: { en: "Cancel", ko: "취소" },
    modal_submit: { en: "Submit", ko: "제출" },
}
