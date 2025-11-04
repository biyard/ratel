import { useTranslation } from 'react-i18next';

export const Refund = {
  en: {
    title: 'Refund Policy',
    lastUpdated: 'Last Updated',
    effectiveDate: 'January 1, 2024',

    sections: {
      introduction: {
        title: '1. Introduction',
        content: 'This Refund Policy outlines the terms and conditions for refunds on Ratel\'s platform. We are committed to providing fair and transparent refund processes for our users. Please read this policy carefully before making any purchases.',
      },
      eligibility: {
        title: '2. Refund Eligibility',
        content: 'Refunds may be requested under the following circumstances:',
        items: [
          'Service disruption or technical issues preventing platform access',
          'Billing errors or duplicate charges',
          'Unauthorized transactions on your account',
          'Cancellation within the cooling-off period (if applicable)',
          'Non-delivery of paid services or features',
        ],
      },
      nonRefundable: {
        title: '3. Non-Refundable Items',
        content: 'The following items and services are generally non-refundable:',
        items: [
          'Completed survey responses and deliberation participations',
          'Digital content that has been accessed or downloaded',
          'Services that have been fully rendered',
          'Promotional or discounted purchases (unless required by law)',
          'Subscription fees after the trial period has ended',
        ],
      },
      requestProcess: {
        title: '4. Refund Request Process',
        content: 'To request a refund, please follow these steps:',
        items: [
          'Contact our support team within the eligible refund period',
          'Provide your order number and payment details',
          'Explain the reason for your refund request',
          'Submit any supporting documentation if required',
          'Wait for our team to review your request (typically 3-5 business days)',
        ],
      },
      processingTime: {
        title: '5. Refund Processing Time',
        content: 'Once your refund request is approved, processing times vary depending on your payment method. Refunds are typically processed within 5-10 business days. Credit card refunds may take an additional 3-5 business days to appear on your statement, depending on your card issuer.',
      },
      partialRefunds: {
        title: '6. Partial Refunds',
        content: 'In certain circumstances, partial refunds may be granted. This may occur when only a portion of the service was affected, or when some features were used while others were not. The refund amount will be calculated proportionally based on the unused portion of the service.',
      },
      subscriptions: {
        title: '7. Subscription Cancellations',
        content: 'For subscription-based services, you may cancel at any time. However, refunds for subscription fees are typically not provided for the current billing period. Cancellation will prevent future charges, and you will retain access until the end of your current billing cycle.',
      },
      chargebacks: {
        title: '8. Chargebacks and Disputes',
        content: 'If you initiate a chargeback or payment dispute with your payment provider, we reserve the right to suspend your account pending resolution. Please contact our support team first to resolve any payment issues, as chargebacks may result in additional fees and account restrictions.',
      },
      exceptions: {
        title: '9. Exceptions and Special Circumstances',
        content: 'We understand that exceptional circumstances may arise. If you believe your situation warrants special consideration, please contact our support team. Each case will be reviewed individually, and we will work to find a fair resolution.',
      },
      modifications: {
        title: '10. Changes to This Policy',
        content: 'We may update this Refund Policy from time to time to reflect changes in our practices or for legal reasons. We will notify you of material changes by posting the updated policy on this page and updating the "Last Updated" date.',
      },
      contact: {
        title: '11. Contact Information',
        content: 'If you have questions about this Refund Policy or wish to request a refund, please contact us at:',
        email: 'Email',
        address: 'Address',
      },
    },
  },
  ko: {
    title: '환불 정책',
    lastUpdated: '최종 업데이트',
    effectiveDate: '2024년 1월 1일',

    sections: {
      introduction: {
        title: '1. 소개',
        content: '본 환불 정책은 Ratel 플랫폼의 환불에 대한 이용 약관을 설명합니다. 당사는 사용자에게 공정하고 투명한 환불 프로세스를 제공하기 위해 최선을 다하고 있습니다. 구매하기 전에 본 정책을 주의 깊게 읽어 주십시오.',
      },
      eligibility: {
        title: '2. 환불 자격',
        content: '다음과 같은 경우 환불을 요청할 수 있습니다:',
        items: [
          '플랫폼 접근을 방해하는 서비스 중단 또는 기술적 문제',
          '청구 오류 또는 중복 청구',
          '귀하의 계정에 대한 무단 거래',
          '유예 기간 내 취소 (해당되는 경우)',
          '유료 서비스 또는 기능의 미제공',
        ],
      },
      nonRefundable: {
        title: '3. 환불 불가 항목',
        content: '다음 항목 및 서비스는 일반적으로 환불되지 않습니다:',
        items: [
          '완료된 설문 응답 및 숙의 참여',
          '액세스되거나 다운로드된 디지털 콘텐츠',
          '완전히 제공된 서비스',
          '프로모션 또는 할인 구매 (법률이 요구하지 않는 한)',
          '체험 기간이 종료된 후의 구독료',
        ],
      },
      requestProcess: {
        title: '4. 환불 요청 절차',
        content: '환불을 요청하려면 다음 단계를 따르십시오:',
        items: [
          '환불 가능 기간 내에 당사 지원팀에 연락',
          '주문 번호 및 결제 세부 정보 제공',
          '환불 요청 사유 설명',
          '필요한 경우 증빙 서류 제출',
          '당사 팀의 검토 대기 (일반적으로 영업일 기준 3-5일)',
        ],
      },
      processingTime: {
        title: '5. 환불 처리 시간',
        content: '환불 요청이 승인되면 결제 방법에 따라 처리 시간이 달라집니다. 환불은 일반적으로 영업일 기준 5-10일 내에 처리됩니다. 신용카드 환불은 카드 발급사에 따라 명세서에 표시되기까지 추가로 3-5 영업일이 소요될 수 있습니다.',
      },
      partialRefunds: {
        title: '6. 부분 환불',
        content: '특정 상황에서는 부분 환불이 승인될 수 있습니다. 이는 서비스의 일부만 영향을 받았거나 일부 기능은 사용되고 다른 기능은 사용되지 않은 경우에 발생할 수 있습니다. 환불 금액은 사용되지 않은 서비스 부분을 기준으로 비례적으로 계산됩니다.',
      },
      subscriptions: {
        title: '7. 구독 취소',
        content: '구독 기반 서비스의 경우 언제든지 취소할 수 있습니다. 그러나 현재 청구 기간에 대한 구독료는 일반적으로 환불되지 않습니다. 취소하면 향후 요금이 청구되지 않으며, 현재 청구 주기가 끝날 때까지 액세스 권한이 유지됩니다.',
      },
      chargebacks: {
        title: '8. 지불 거절 및 분쟁',
        content: '결제 제공업체에 지불 거절 또는 결제 분쟁을 제기하는 경우, 당사는 해결이 될 때까지 귀하의 계정을 정지할 권리를 보유합니다. 지불 거절은 추가 수수료 및 계정 제한을 초래할 수 있으므로 결제 문제를 해결하기 위해 먼저 당사 지원팀에 연락해 주십시오.',
      },
      exceptions: {
        title: '9. 예외 및 특별한 상황',
        content: '예외적인 상황이 발생할 수 있음을 이해합니다. 귀하의 상황이 특별한 고려를 필요로 한다고 생각되면 당사 지원팀에 연락해 주십시오. 각 사례는 개별적으로 검토되며, 공정한 해결책을 찾기 위해 노력할 것입니다.',
      },
      modifications: {
        title: '10. 정책 변경',
        content: '당사는 관행의 변경 또는 법적 사유로 인해 수시로 본 환불 정책을 업데이트할 수 있습니다. 이 페이지에 업데이트된 정책을 게시하고 "최종 업데이트" 날짜를 업데이트하여 중요한 변경 사항을 알려드립니다.',
      },
      contact: {
        title: '11. 연락처 정보',
        content: '본 환불 정책에 대해 질문이 있거나 환불을 요청하려면 다음으로 연락해 주십시오:',
        email: '이메일',
        address: '주소',
      },
    },
  },
};

export interface RefundI18n {
  title: string;
  lastUpdated: string;
  effectiveDate: string;
  sections: {
    introduction: { title: string; content: string };
    eligibility: { title: string; content: string; items: string[] };
    nonRefundable: { title: string; content: string; items: string[] };
    requestProcess: { title: string; content: string; items: string[] };
    processingTime: { title: string; content: string };
    partialRefunds: { title: string; content: string };
    subscriptions: { title: string; content: string };
    chargebacks: { title: string; content: string };
    exceptions: { title: string; content: string };
    modifications: { title: string; content: string };
    contact: { title: string; content: string; email: string; address: string };
  };
}

export function useRefundI18n(): RefundI18n {
  const { t } = useTranslation('Refund');

  return {
    title: t('title'),
    lastUpdated: t('lastUpdated'),
    effectiveDate: t('effectiveDate'),
    sections: {
      introduction: {
        title: t('sections.introduction.title'),
        content: t('sections.introduction.content'),
      },
      eligibility: {
        title: t('sections.eligibility.title'),
        content: t('sections.eligibility.content'),
        items: t('sections.eligibility.items', { returnObjects: true }) as string[],
      },
      nonRefundable: {
        title: t('sections.nonRefundable.title'),
        content: t('sections.nonRefundable.content'),
        items: t('sections.nonRefundable.items', { returnObjects: true }) as string[],
      },
      requestProcess: {
        title: t('sections.requestProcess.title'),
        content: t('sections.requestProcess.content'),
        items: t('sections.requestProcess.items', { returnObjects: true }) as string[],
      },
      processingTime: {
        title: t('sections.processingTime.title'),
        content: t('sections.processingTime.content'),
      },
      partialRefunds: {
        title: t('sections.partialRefunds.title'),
        content: t('sections.partialRefunds.content'),
      },
      subscriptions: {
        title: t('sections.subscriptions.title'),
        content: t('sections.subscriptions.content'),
      },
      chargebacks: {
        title: t('sections.chargebacks.title'),
        content: t('sections.chargebacks.content'),
      },
      exceptions: {
        title: t('sections.exceptions.title'),
        content: t('sections.exceptions.content'),
      },
      modifications: {
        title: t('sections.modifications.title'),
        content: t('sections.modifications.content'),
      },
      contact: {
        title: t('sections.contact.title'),
        content: t('sections.contact.content'),
        email: t('sections.contact.email'),
        address: t('sections.contact.address'),
      },
    },
  };
}
