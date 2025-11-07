import { useTranslation } from 'react-i18next';

export const Credentials = {
  en: {
    vc: 'Verifiable Credential',
    id: 'ID',
    my_did: 'My DID',
    age: 'Age',
    gender: 'Gender',
    university: 'University',
    verified: 'Verified',
    verify: 'Verify',
    registration_required: 'Registration required',
    age_range: '20 - 29',
    kaia: 'KAIA',
    male: 'Male',
    female: 'Female',
    no_data: 'No verified data available.',
    select_verification_method: 'Select Verification Method',
    identity_verification: 'Identity Verification',
    identity_verification_desc: 'Verify via official identification',
    code_verification: 'Code Verification',
    code_verification_desc: 'Verify using pre-issued code',
    cancel: 'Cancel',
    enter_code: 'Enter Verification Code',
    code_placeholder: 'Enter your code',
    submit: 'Submit',
    submitting: 'Verifying...',
    verification_success: 'Verification successful!',
    verification_error: 'Verification failed',
    invalid_code: 'Invalid code. Please try again.',
  },
  ko: {
    vc: '검증가능한 자격 증명',
    id: 'ID',
    my_did: 'My DID',
    age: '나이',
    gender: '성별',
    university: '대학교',
    verified: '인증됨',
    verify: '인증하기',
    registration_required: '등록 필요',
    age_range: '20 - 29',
    kaia: 'KAIA',
    male: '남성',
    female: '여성',
    no_data: '검증된 데이터가 없습니다.',
    select_verification_method: '인증 방법 선택',
    identity_verification: '본인 인증',
    identity_verification_desc: '공식 신분증을 통한 인증',
    code_verification: '코드 인증',
    code_verification_desc: '미리 발급받은 코드로 인증',
    cancel: '취소',
    enter_code: '인증 코드 입력',
    code_placeholder: '코드를 입력하세요',
    submit: '제출',
    submitting: '인증 중...',
    verification_success: '인증이 완료되었습니다!',
    verification_error: '인증에 실패했습니다',
    invalid_code: '유효하지 않은 코드입니다. 다시 시도해주세요.',
  },
};

export interface CredentialsI18n {
  vc: string;
  id: string;
  my_did: string;

  age: string;
  gender: string;
  university: string;

  verified: string;
  verify: string;
  registration_required: string;
  age_range: string;
  kaia: string;
  male: string;
  female: string;
  no_data: string;
  selectVerificationMethod: string;
  identityVerification: string;
  identityVerificationDesc: string;
  codeVerification: string;
  codeVerificationDesc: string;
  cancel: string;
  enterCode: string;
  codePlaceholder: string;
  submit: string;
  submitting: string;
  verificationSuccess: string;
  verificationError: string;
  invalidCode: string;
}

export function useCredentialsI18n(): CredentialsI18n {
  const { t } = useTranslation('Credentials');

  return {
    vc: t('vc'),
    id: t('id'),
    my_did: t('my_did'),
    age: t('age'),
    gender: t('gender'),
    university: t('university'),
    verified: t('verified'),
    verify: t('verify'),
    registration_required: t('registration_required'),
    age_range: t('age_range'),
    kaia: t('kaia'),
    male: t('male'),
    female: t('female'),
    no_data: t('no_data'),
    selectVerificationMethod: t('select_verification_method'),
    identityVerification: t('identity_verification'),
    identityVerificationDesc: t('identity_verification_desc'),
    codeVerification: t('code_verification'),
    codeVerificationDesc: t('code_verification_desc'),
    cancel: t('cancel'),
    enterCode: t('enter_code'),
    codePlaceholder: t('code_placeholder'),
    submit: t('submit'),
    submitting: t('submitting'),
    verificationSuccess: t('verification_success'),
    verificationError: t('verification_error'),
    invalidCode: t('invalid_code'),
  };
}
