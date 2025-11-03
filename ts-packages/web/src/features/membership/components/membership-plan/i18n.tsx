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
