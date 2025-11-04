import { useTranslation } from 'react-i18next';

export const Credentials = {
  en: {
    vc: 'Verifiable Credential',
    id: 'ID',
    my_did: 'My DID',
    age: 'Age',
    gender: 'Gender',
    verified: 'Verified',
    verify: 'Verify',
    registration_required: 'Registration required',
    age_range: '20 - 29',
    kaia: 'KAIA',
    male: 'Male',
    female: 'Female',
    no_data: 'No verified data available.',
  },
  ko: {
    vc: '검증가능한 자격 증명',
    id: 'ID',
    my_did: 'My DID',
    age: '나이',
    gender: '성별',
    verified: '인증됨',
    verify: '인증하기',
    registration_required: '등록 필요',
    age_range: '20 - 29',
    kaia: 'KAIA',
    male: '남성',
    female: '여성',
    no_data: '검증된 데이터가 없습니다.',
  },
};

export interface CredentialsI18n {
  vc: string;
  id: string;
  my_did: string;
  age: string;
  gender: string;
  verified: string;
  verify: string;
  registration_required: string;
  age_range: string;
  kaia: string;
  male: string;
  female: string;
  no_data: string;
}

export function useCredentialsI18n(): CredentialsI18n {
  const { t } = useTranslation('Credentials');

  return {
    vc: t('vc'),
    id: t('id'),
    my_did: t('my_did'),
    age: t('age'),
    gender: t('gender'),
    verified: t('verified'),
    verify: t('verify'),
    registration_required: t('registration_required'),
    age_range: t('age_range'),
    kaia: t('kaia'),
    male: t('male'),
    female: t('female'),
    no_data: t('no_data'),
  };
}
