use crate::*;

translate! {
    MembershipPlanTranslate;

    title: {
        en: "Membership Plans",
        ko: "멤버십 플랜",
    },

    description: {
        en: "<strong class=\"font-bold text-primary\">Credits</strong> are monthly points you can use to create or boost <span class=\"text-primary\">Reward Spaces</span>.",
        ko: "<strong class=\"font-bold text-primary\">Credits</strong>은 <span class=\"text-primary\">보상 스페이스</span>를 생성하거나 부스팅시키는 데 사용할 수 있는 월간 포인트입니다.",
    },

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MembershipTier {
    Free,
    Pro,
    Max,
    Vip,
    Enterprise,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MembershipPlanItem {
    pub tier: MembershipTier,
    pub name: &'static str,
    pub description: &'static str,
    pub features: Vec<&'static str>,
    pub price: Option<&'static str>,
    pub btn: Option<&'static str>,
    pub credits: Option<i64>,
}

pub fn membership_plan_items(is_ko: bool) -> Vec<MembershipPlanItem> {
    if is_ko {
        vec![
            MembershipPlanItem {
                tier: MembershipTier::Free,
                name: "무료",
                description: "누구나 참여 가능한 기본 멤버십",
                features: vec![
                    "포스트 게재",
                    "스페이스 생성",
                    "인맥 관리",
                    "보상 스페이스 참여",
                ],
                price: None,
                btn: None,
                credits: None,
            },
            MembershipPlanItem {
                tier: MembershipTier::Pro,
                name: "Pro",
                description: "Reward Space setup for small communities",
                features: vec![
                    "모든 무료 플랜 포함",
                    "월별 40 크레딧 제공",
                    "보상 스페이스 또는 보상 기능별 최대 2 크레딧 사용 가능",
                    "참여자 전체 보상의 10% 생성 보상 획득",
                ],
                price: Some("월 30,000원"),
                btn: Some("Pro 신청"),
                credits: Some(40),
            },
            MembershipPlanItem {
                tier: MembershipTier::Max,
                name: "Max",
                description: "대규모 커뮤니티를 위한 보상 스페이스 기능 제공",
                features: vec![
                    "모든 무료 플랜 포함",
                    "월별 190 크레딧 제공",
                    "보상 스페이스 또는 보상 기능별 최대 10 크레딧 사용 가능",
                    "참여자 전체 보상의 10% 생성 보상 획득",
                    "신뢰 크리에이터 배지 획득",
                ],
                price: Some("월 75,000원"),
                btn: Some("Max 신청"),
                credits: Some(190),
            },
            MembershipPlanItem {
                tier: MembershipTier::Vip,
                name: "VIP",
                description: "인플루언서 및 마케팅 전문 기업을 위한 보상 스페이스 기능 제공",
                features: vec![
                    "모든 무료 플랜 포함",
                    "월별 1,360 크레딧 제공",
                    "보상 스페이스 또는 보상 기능별 최대 100 크레딧 사용 가능",
                    "참여자 전체 보상의 10% 생성 보상 획득",
                    "신뢰 크리에이터 배지 획득",
                    "참여자 원본 데이터 열람",
                ],
                price: Some("월 150,000원"),
                btn: Some("VIP 신청"),
                credits: Some(1360),
            },
            MembershipPlanItem {
                tier: MembershipTier::Enterprise,
                name: "엔터프라이즈",
                description: "기업 및 기관 맞춤형 파트너 멤버쉽",
                features: vec!["모든 무료 플랜 포함", "완전 맞춤형 서비스 제공"],
                price: Some("월 1,000,000원 이상"),
                btn: Some("Contact Us"),
                credits: None,
            },
        ]
    } else {
        vec![
            MembershipPlanItem {
                tier: MembershipTier::Free,
                name: "Free",
                description: "Basic membership open to everyone",
                features: vec![
                    "Publish posts",
                    "Publish spaces",
                    "Network relationship",
                    "Participate reward spaces",
                ],
                price: None,
                btn: None,
                credits: None,
            },
            MembershipPlanItem {
                tier: MembershipTier::Pro,
                name: "Pro",
                description: "Reward Space setup for small communities",
                features: vec![
                    "Includes all Free",
                    "40 monthly credits",
                    "Up to 2 credits per a reward space",
                    "Earn 10% of the total rewards distributed to participants.",
                ],
                price: Some("₩30,000 / month"),
                btn: Some("Get Pro"),
                credits: Some(40),
            },
            MembershipPlanItem {
                tier: MembershipTier::Max,
                name: "Max",
                description: "Advanced Reward Spaces for large communities ",
                features: vec![
                    "Includes all Free",
                    "190 monthly credits",
                    "Up to 10 credits per a reward space",
                    "Earn 10% of the total rewards distributed to participants.",
                    "Get a trusted creator badge",
                ],
                price: Some("₩75,000 / month"),
                btn: Some("Get Max"),
                credits: Some(190),
            },
            MembershipPlanItem {
                tier: MembershipTier::Vip,
                name: "VIP",
                description: "Reward Spaces for influencers and promotion ",
                features: vec![
                    "Includes all Free",
                    "1,360 monthly credits",
                    "Up to 100 credits per a reward space",
                    "Earn 10% of the total rewards distributed to participants.",
                    "Get a trusted creator badge",
                    "Access raw participant data",
                ],
                price: Some("₩150,000 / month"),
                btn: Some("Get VIP"),
                credits: Some(1360),
            },
            MembershipPlanItem {
                tier: MembershipTier::Enterprise,
                name: "Enterprise",
                description: "Customized partner plan for enterprises & organizations",
                features: vec!["Includes all Free", "Fully customization"],
                price: Some("Starting at $1,000 / month"),
                btn: Some("Contact Us"),
                credits: None,
            },
        ]
    }
}
