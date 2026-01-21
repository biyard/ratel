import { useTranslation } from 'react-i18next';

export const MembershipPlan = {
  en: {
    title: 'Membership Plans',
    desciption:
      '<strong class="font-bold text-primary">Credits</strong> are monthly points you can use to create or boost <span class="text-primary">Reward Spaces</span>.',
    purchaseModal: {
      birthDate: 'Birth Date',
      membershipLabel: 'Membership',
      monthlySubscription: 'Monthly subscription',
      fullNameLabel: 'Full Name *',
      fullNamePlaceholder: 'Enter your full name',
      emailLabel: 'Email Address *',
      emailPlaceholder: 'Enter your email',
      phoneLabel: 'Phone Number *',
      phonePlaceholder: 'Enter your phone number',
      cardInformationTitle: 'Card Information',
      cardNumberLabel: 'Card Number *',
      cardNumberPlaceholder: 'Enter card number',
      expiryLabel: 'Expiry Date *',
      expiryMonthPlaceholder: 'MM',
      expiryYearPlaceholder: 'YY',
      birthOrBizLabel: 'Birth Date / Business Registration Number *',
      birthOrBizPlaceholder: 'YYMMDD or 10 digits',
      cardPasswordLabel: 'Card Password (first 2 digits) *',
      cardPasswordPlaceholder: '••',
      cancelButton: 'Cancel',
      confirmButton: 'Proceed to Payment',
    },
    receiptModal: {
      title: 'Payment Successful!',
      thankYouMessage: 'Thank you for your purchase',
      transactionIdLabel: 'Transaction ID',
      membershipLabel: 'Membership',
      amountLabel: 'Amount Paid',
      durationLabel: 'Duration',
      creditsLabel: 'Credits',
      paidAtLabel: 'Payment Date',
      daysLabel: 'days',
      closeButton: 'Close',
    },
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
        price: '₩30,000 / month',
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
        price: '₩75,000 / month',
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
        price: '₩150,000 / month',
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
    male: '남성',
    female: '여성',
    purchaseModal: {
      birthDate: '생년월일',
      membershipLabel: '멤버십',
      monthlySubscription: '월간 구독',
      fullNameLabel: '성명 *',
      fullNamePlaceholder: '성명을 입력하세요',
      emailLabel: '이메일 주소 *',
      emailPlaceholder: '이메일을 입력하세요',
      phoneLabel: '전화번호 *',
      phonePlaceholder: '전화번호를 입력하세요',
      cardInformationTitle: '카드 정보',
      cardNumberLabel: '카드 번호 *',
      cardNumberPlaceholder: '카드 번호를 입력하세요',
      expiryLabel: '유효 기간 *',
      expiryMonthPlaceholder: 'MM',
      expiryYearPlaceholder: 'YY',
      birthOrBizLabel: '생년월일 / 사업자등록번호 *',
      birthOrBizPlaceholder: 'YYMMDD 또는 10자리',
      cardPasswordLabel: '카드 비밀번호 앞 2자리 *',
      cardPasswordPlaceholder: '••',
      cancelButton: '취소',
      confirmButton: '결제 진행',
    },
    receiptModal: {
      title: '결제 완료!',
      thankYouMessage: '구매해 주셔서 감사합니다',
      transactionIdLabel: '거래 번호',
      membershipLabel: '멤버십',
      amountLabel: '결제 금액',
      durationLabel: '기간',
      creditsLabel: '크레딧',
      paidAtLabel: '결제 날짜',
      daysLabel: '일',
      closeButton: '닫기',
    },
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
        price: '월 30,000원',
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
        price: '월 75,000원',
        btn: 'Max 신청',
      },
      {
        name: 'VIP',
        description:
          '인플루언서 및 마케팅 전문 기업을 위한 보상 스페이스 기능 제공',
        features: [
          '모든 무료 플랜 포함',
          '월별 1,360 크레딧 제공',
          '보상 스페이스 또는 보상 기능별 최대 100 크레딧 사용 가능',
          '참여자 전체 보상의 10% 생성 보상 획득',
          '신뢰 크리에이터 배지 획득',
          '참여자 원본 데이터 열람',
        ],
        price: '월 150,000원',
        btn: 'VIP 신청',
      },
      {
        name: '엔터프라이즈',
        description: '기업 및 기관 맞춤형 파트너 멤버쉽',
        features: ['모든 무료 플랜 포함', '완전 맞춤형 서비스 제공'],
        price: '월 1,000,000원 이상',
        btn: 'Contact Us',
      },
    ],
  },
};

export interface MembershipPlanI18n {
  title: string;
  description: { __html: string };
  purchaseModal: PurchaseModalI18n;
  receiptModal: ReceiptModalI18n;
  memberships: Array<MembershipPlanItem>;
}

export interface PurchaseModalI18n {
  membershipLabel: string;
  monthlySubscription: string;
  fullNameLabel: string;
  fullNamePlaceholder: string;
  emailLabel: string;
  emailPlaceholder: string;
  phoneLabel: string;
  phonePlaceholder: string;
  cardInformationTitle: string;
  cardNumberLabel: string;
  cardNumberPlaceholder: string;
  expiryLabel: string;
  expiryMonthPlaceholder: string;
  expiryYearPlaceholder: string;
  birthOrBizLabel: string;
  birthOrBizPlaceholder: string;
  cardPasswordLabel: string;
  cardPasswordPlaceholder: string;
  cancelButton: string;
  confirmButton: string;
  birthDate: string;
}

export interface ReceiptModalI18n {
  title: string;
  thankYouMessage: string;
  transactionIdLabel: string;
  membershipLabel: string;
  amountLabel: string;
  durationLabel: string;
  creditsLabel: string;
  paidAtLabel: string;
  daysLabel: string;
  closeButton: string;
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
    purchaseModal: t('purchaseModal', {
      returnObjects: true,
    }) as PurchaseModalI18n,
    receiptModal: t('receiptModal', {
      returnObjects: true,
    }) as ReceiptModalI18n,
    memberships: t('memberships', {
      returnObjects: true,
    }) as Array<MembershipPlanItem>,
  };
}
