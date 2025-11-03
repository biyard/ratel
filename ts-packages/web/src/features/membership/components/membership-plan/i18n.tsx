import { useTranslation } from 'react-i18next';

export const MembershipPlan = {
  en: {
    title: 'Membership Plans',
    desciption:
      '<strong class="font-bold text-primary">Credits</strong> are monthly points you can use to create or boost <span class="text-primary">Reward Spaces</span>.',
    memberships: [
      {
        name: 'Free',
        description: 'Basic membership open to everyone',
        features: [
          'Publish posts',
          'Publish spaces',
          'Network relationship',
          'Participate reward spaces',
        ],
      },
      {
        name: 'Pro',
        description: 'Reward Space setup for small communities',
        features: [
          'Includes all Free',
          '40 monthly credits',
          'Up to 2 credits per a reward space',
          'Earn 10% of the total rewards distributed to participants.',
        ],
        price: '$20 / month',
        btn: 'Get Pro',
      },
      {
        name: 'Max',
        description: 'Advanced Reward Spaces for large communities ',
        features: [
          'Includes all Free',
          '190 monthly credits',
          'Up to 10 credits per a reward space',
          'Earn 10% of the total rewards distributed to participants.',
          'Get a trusted creator badge',
        ],
        price: '$50 / month',
        btn: 'Get Max',
      },
      {
        name: 'VIP',
        description: 'Reward Spaces for influencers and promotion ',
        features: [
          'Includes all Free',
          '1,360 monthly credits',
          'Up to 100 credits per a reward space',
          'Earn 10% of the total rewards distributed to participants.',
          'Get a trusted creator badge',
          'Access raw participant data',
        ],
        price: '$100 / month',
        btn: 'Get VIP',
      },
      {
        name: 'Enterprise',
        description: 'Customized partner plan for enterprises & organizations',
        features: ['Includes all Free', 'Fully customization'],
        price: 'Starting at $1,000 / month',
        btn: 'Contact Us',
      },
    ],
  },
  ko: {
    title: '멤버십 플랜',
    desciption:
      '<strong class="font-bold text-primary">Credits</strong>은 <span class="text-primary">보상 스페이스</span>를 생성하거나 부스팅시키는 데 사용할 수 있는 월간 포인트입니다.',
    memberships: [
      {
        name: '무료',
        description: '누구나 참여 가능한 기본 멤버십',
        features: [
          '포스트 게재',
          '스페이스 생성',
          '인맥 관리',
          '보상 스페이스 참여',
        ],
      },
      {
        name: 'Pro',
        description: 'Reward Space setup for small communities',
        features: [
          '모든 무료 플랜 포함',
          '월별 40 크레딧 제공',
          '보상 스페이스 또는 보상 기능별 최대 2 크레딧 사용 가능',
          '참여자 전체 보상의 10% 생성 보상 획득',
        ],
        price: '월 $20',
        btn: 'Pro 신청',
      },
      {
        name: 'Max',
        description: '대규모 커뮤니티를 위한 보상 스페이스 기능 제공',
        features: [
          '모든 무료 플랜 포함',
          '월별 190 크레딧 제공',
          '보상 스페이스 또는 보상 기능별 최대 10 크레딧 사용 가능',
          '참여자 전체 보상의 10% 생성 보상 획득',
          '신뢰 크리에이터 배지 획득',
        ],
        price: '월 $50',
        btn: 'Max 신청',
      },
      {
        name: 'VIP',
        description:
          '인플루언서 및 마케팅 전문 기업를 위한 보상 스페이스 기능 제공',
        features: [
          '모든 무료 플랜 포함',
          '월별 1,360 크레딧 제공',
          '보상 스페이스 또는 보상 기능별 최대 100 크레딧 사용 가능',
          '참여자 전체 보상의 10% 생성 보상 획득',
          '신뢰 크리에이터 배지 획득',
          '참여자 원본 데이터 열람',
        ],
        price: '월 $100',
        btn: 'VIP 신청',
      },
      {
        name: '엔터프라이즈',
        description: '기업 및 기관 맞춤형 파트너 멤버쉽',
        features: ['모든 무료 플랜 포함', '완전 맞춤형 서비스 제공'],
        price: '월 $1,000 이상',
        btn: 'Contact Us',
      },
    ],
  },
};

export interface MembershipPlanI18n {
  title: string;
  description: { __html: string };
  memberships: Array<MembershipPlanItem>;
}
export interface MembershipPlanItem {
  name: string;
  description: string;
  features: string[];
  price?: string;
  btn?: string;
}

export function useMembershipPlanI18n(): MembershipPlanI18n {
  const { t } = useTranslation('MembershipPlan');

  return {
    title: t('title'),
    description: { __html: t('desciption') },
    memberships: t('memberships', {
      returnObjects: true,
    }) as Array<MembershipPlanItem>,
  };
}
