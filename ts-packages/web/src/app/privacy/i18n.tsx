import { useTranslation } from 'react-i18next';

export const Privacy = {
  en: {
    title: 'Privacy Policy',
    lastUpdated: 'Last Updated',
    effectiveDate: 'January 1, 2024',

    sections: {
      introduction: {
        title: '1. Introduction',
        content:
          'This Privacy Policy explains how Ratel ("we," "us," or "our") collects, uses, discloses, and protects your personal information when you use our survey and deliberation platform. We are committed to protecting your privacy and ensuring the security of your personal data.',
      },
      informationCollection: {
        title: '2. Information We Collect',
        content:
          'We collect information that you provide directly to us, including:',
        items: [
          'Account information (name, email address, username)',
          'Profile information (bio, profile picture, preferences)',
          'Content you create (posts, comments, survey responses, votes)',
          'Communication data (messages, notifications)',
          'Usage data (interactions, features used, time spent)',
          'Device information (IP address, browser type, operating system)',
          'Connection Information (CI): Network connection data including session information, login timestamps, access locations, and connection patterns to ensure service quality and security',
          'Duplication Information (DI): Data used to detect and prevent duplicate accounts, spam content, or fraudulent activities including account fingerprints, behavioral patterns, and content similarity analysis',
        ],
      },
      howWeUseInfo: {
        title: '3. How We Use Your Information',
        content: 'We use the collected information for the following purposes:',
        items: [
          'To provide, maintain, and improve our services',
          'To process your survey responses and facilitate deliberation',
          'To communicate with you about updates and features',
          'To personalize your experience',
          'To analyze usage patterns and improve our platform',
          'To detect, prevent, and address security issues',
          'To comply with legal obligations',
        ],
      },
      informationSharing: {
        title: '4. Information Sharing and Disclosure',
        content:
          'We do not sell your personal information. We may share your information in the following circumstances:',
        items: [
          'With your consent or at your direction',
          'With service providers who assist in operating our platform',
          'To comply with legal obligations or respond to lawful requests',
          'To protect our rights, property, or safety',
          'In connection with a business transfer or merger',
        ],
      },
      dataSecurity: {
        title: '5. Data Security',
        content:
          'We implement appropriate technical and organizational measures to protect your personal information against unauthorized access, alteration, disclosure, or destruction. However, no method of transmission over the Internet is 100% secure.',
      },
      dataRetention: {
        title: '6. Data Retention',
        content:
          'We retain your personal information for as long as necessary to provide our services and fulfill the purposes outlined in this Privacy Policy. You may request deletion of your account and associated data at any time.',
      },
      yourRights: {
        title: '7. Your Rights',
        content:
          'You have the following rights regarding your personal information:',
        items: [
          'Access: Request access to your personal data',
          'Correction: Request correction of inaccurate data',
          'Deletion: Request deletion of your data',
          'Portability: Request a copy of your data in a portable format',
          'Objection: Object to certain processing of your data',
          'Withdrawal: Withdraw consent at any time',
        ],
      },
      cookies: {
        title: '8. Cookies and Tracking',
        content:
          'We use cookies and similar tracking technologies to enhance your experience, analyze usage, and personalize content. You can control cookies through your browser settings.',
      },
      thirdPartyLinks: {
        title: '9. Third-Party Links',
        content:
          'Our platform may contain links to third-party websites or services. We are not responsible for the privacy practices of these third parties. We encourage you to review their privacy policies.',
      },
      childrenPrivacy: {
        title: "10. Children's Privacy",
        content:
          'Our services are not intended for children under 13 years of age. We do not knowingly collect personal information from children under 13. If you believe we have collected such information, please contact us.',
      },
      internationalTransfers: {
        title: '11. International Data Transfers',
        content:
          'Your information may be transferred to and processed in countries other than your country of residence. We ensure appropriate safeguards are in place to protect your data.',
      },
      changes: {
        title: '12. Changes to This Policy',
        content:
          'We may update this Privacy Policy from time to time. We will notify you of material changes by posting the updated policy on this page and updating the "Last Updated" date.',
      },
      contact: {
        title: '13. Contact Us',
        content:
          'If you have questions or concerns about this Privacy Policy or our data practices, please contact us at:',
        email: 'Email',
        address: 'Address',
      },
    },
  },
  ko: {
    title: '개인정보처리방침',
    lastUpdated: '최종 업데이트',
    effectiveDate: '2024년 1월 1일',

    sections: {
      introduction: {
        title: '1. 소개',
        content:
          '본 개인정보처리방침은 Ratel("당사", "우리")이 설문조사 및 숙의 플랫폼을 이용할 때 귀하의 개인정보를 어떻게 수집, 사용, 공개 및 보호하는지 설명합니다. 당사는 귀하의 개인정보를 보호하고 개인 데이터의 보안을 보장하기 위해 최선을 다하고 있습니다.',
      },
      informationCollection: {
        title: '2. 수집하는 정보',
        content: '당사는 귀하가 직접 제공하는 정보를 수집합니다:',
        items: [
          '계정 정보 (이름, 이메일 주소, 사용자 이름)',
          '프로필 정보 (자기소개, 프로필 사진, 환경설정)',
          '생성한 콘텐츠 (게시물, 댓글, 설문 응답, 투표)',
          '커뮤니케이션 데이터 (메시지, 알림)',
          '사용 데이터 (상호작용, 사용한 기능, 사용 시간)',
          '기기 정보 (IP 주소, 브라우저 유형, 운영체제)',
          '암호화된 이용자 확인값 (CI): 서비스 품질과 보안을 보장하기 위한 사용자 정보',
          '중복가입확인정보 (DI): 계정 핑거프린트, 행동 패턴, 콘텐츠 유사성 분석을 포함하여 중복 계정, 스팸 콘텐츠 또는 사기 활동을 탐지하고 방지하는 데 사용되는 데이터',
        ],
      },
      howWeUseInfo: {
        title: '3. 정보 사용 방법',
        content: '당사는 수집된 정보를 다음 목적으로 사용합니다:',
        items: [
          '서비스 제공, 유지 및 개선',
          '설문 응답 처리 및 숙의 촉진',
          '업데이트 및 기능에 대한 소통',
          '개인화된 경험 제공',
          '사용 패턴 분석 및 플랫폼 개선',
          '보안 문제 탐지, 방지 및 해결',
          '법적 의무 준수',
        ],
      },
      informationSharing: {
        title: '4. 정보 공유 및 공개',
        content:
          '당사는 귀하의 개인정보를 판매하지 않습니다. 다음과 같은 경우 귀하의 정보를 공유할 수 있습니다:',
        items: [
          '귀하의 동의 또는 지시에 따라',
          '플랫폼 운영을 지원하는 서비스 제공업체와',
          '법적 의무 준수 또는 합법적 요청에 응답하기 위해',
          '당사의 권리, 재산 또는 안전을 보호하기 위해',
          '사업 양도 또는 합병과 관련하여',
        ],
      },
      dataSecurity: {
        title: '5. 데이터 보안',
        content:
          '당사는 귀하의 개인정보를 무단 액세스, 변경, 공개 또는 파괴로부터 보호하기 위해 적절한 기술적 및 조직적 조치를 구현합니다. 그러나 인터넷을 통한 전송 방법은 100% 안전하지 않습니다.',
      },
      dataRetention: {
        title: '6. 데이터 보관',
        content:
          '당사는 서비스를 제공하고 본 개인정보처리방침에 명시된 목적을 달성하는 데 필요한 기간 동안 귀하의 개인정보를 보관합니다. 귀하는 언제든지 계정 및 관련 데이터의 삭제를 요청할 수 있습니다.',
      },
      yourRights: {
        title: '7. 귀하의 권리',
        content: '귀하는 개인정보와 관련하여 다음과 같은 권리를 가집니다:',
        items: [
          '열람: 개인 데이터에 대한 액세스 요청',
          '정정: 부정확한 데이터의 정정 요청',
          '삭제: 데이터 삭제 요청',
          '이동: 이동 가능한 형식으로 데이터 사본 요청',
          '반대: 특정 데이터 처리에 대한 반대',
          '철회: 언제든지 동의 철회',
        ],
      },
      cookies: {
        title: '8. 쿠키 및 추적',
        content:
          '당사는 경험을 향상시키고, 사용을 분석하고, 콘텐츠를 개인화하기 위해 쿠키 및 유사한 추적 기술을 사용합니다. 브라우저 설정을 통해 쿠키를 제어할 수 있습니다.',
      },
      thirdPartyLinks: {
        title: '9. 제3자 링크',
        content:
          '당사의 플랫폼에는 제3자 웹사이트 또는 서비스에 대한 링크가 포함될 수 있습니다. 당사는 이러한 제3자의 개인정보 보호 관행에 대해 책임을 지지 않습니다. 그들의 개인정보처리방침을 검토할 것을 권장합니다.',
      },
      childrenPrivacy: {
        title: '10. 아동 개인정보 보호',
        content:
          '당사의 서비스는 13세 미만의 아동을 대상으로 하지 않습니다. 당사는 13세 미만 아동의 개인정보를 고의로 수집하지 않습니다. 당사가 그러한 정보를 수집했다고 생각되면 연락해 주십시오.',
      },
      internationalTransfers: {
        title: '11. 국제 데이터 전송',
        content:
          '귀하의 정보는 귀하의 거주 국가 이외의 국가로 전송되어 처리될 수 있습니다. 당사는 귀하의 데이터를 보호하기 위해 적절한 안전 조치를 취하고 있습니다.',
      },
      changes: {
        title: '12. 방침 변경',
        content:
          '당사는 수시로 본 개인정보처리방침을 업데이트할 수 있습니다. 이 페이지에 업데이트된 방침을 게시하고 "최종 업데이트" 날짜를 업데이트하여 중요한 변경 사항을 알려드립니다.',
      },
      contact: {
        title: '13. 문의하기',
        content:
          '본 개인정보처리방침 또는 당사의 데이터 관행에 대해 질문이나 우려 사항이 있으시면 다음으로 연락해 주십시오:',
        email: '이메일',
        address: '주소',
      },
    },
  },
};

export interface PrivacyI18n {
  title: string;
  lastUpdated: string;
  effectiveDate: string;
  sections: {
    introduction: { title: string; content: string };
    informationCollection: { title: string; content: string; items: string[] };
    howWeUseInfo: { title: string; content: string; items: string[] };
    informationSharing: { title: string; content: string; items: string[] };
    dataSecurity: { title: string; content: string };
    dataRetention: { title: string; content: string };
    yourRights: { title: string; content: string; items: string[] };
    cookies: { title: string; content: string };
    thirdPartyLinks: { title: string; content: string };
    childrenPrivacy: { title: string; content: string };
    internationalTransfers: { title: string; content: string };
    changes: { title: string; content: string };
    contact: { title: string; content: string; email: string; address: string };
  };
}

export function usePrivacyI18n(): PrivacyI18n {
  const { t } = useTranslation('Privacy');

  return {
    title: t('title'),
    lastUpdated: t('lastUpdated'),
    effectiveDate: t('effectiveDate'),
    sections: {
      introduction: {
        title: t('sections.introduction.title'),
        content: t('sections.introduction.content'),
      },
      informationCollection: {
        title: t('sections.informationCollection.title'),
        content: t('sections.informationCollection.content'),
        items: t('sections.informationCollection.items', {
          returnObjects: true,
        }) as string[],
      },
      howWeUseInfo: {
        title: t('sections.howWeUseInfo.title'),
        content: t('sections.howWeUseInfo.content'),
        items: t('sections.howWeUseInfo.items', {
          returnObjects: true,
        }) as string[],
      },
      informationSharing: {
        title: t('sections.informationSharing.title'),
        content: t('sections.informationSharing.content'),
        items: t('sections.informationSharing.items', {
          returnObjects: true,
        }) as string[],
      },
      dataSecurity: {
        title: t('sections.dataSecurity.title'),
        content: t('sections.dataSecurity.content'),
      },
      dataRetention: {
        title: t('sections.dataRetention.title'),
        content: t('sections.dataRetention.content'),
      },
      yourRights: {
        title: t('sections.yourRights.title'),
        content: t('sections.yourRights.content'),
        items: t('sections.yourRights.items', {
          returnObjects: true,
        }) as string[],
      },
      cookies: {
        title: t('sections.cookies.title'),
        content: t('sections.cookies.content'),
      },
      thirdPartyLinks: {
        title: t('sections.thirdPartyLinks.title'),
        content: t('sections.thirdPartyLinks.content'),
      },
      childrenPrivacy: {
        title: t('sections.childrenPrivacy.title'),
        content: t('sections.childrenPrivacy.content'),
      },
      internationalTransfers: {
        title: t('sections.internationalTransfers.title'),
        content: t('sections.internationalTransfers.content'),
      },
      changes: {
        title: t('sections.changes.title'),
        content: t('sections.changes.content'),
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
