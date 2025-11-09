import { useTranslation } from 'react-i18next';

export const i18nAttributeCodes = {
  en: {
    title: 'Attribute Code Management',
    create_new: 'Create New Code',
    loading: 'Loading...',
    load_error: 'Load Failed',
    code: 'Code',
    created_at: 'Created At',
    attributes: 'Attributes',
    actions: 'Actions',
    delete: 'Delete',
    delete_confirm: 'Are you sure you want to delete this attribute code?',
    cancel: 'Cancel',
    confirm: 'Confirm',
    create_title: 'Create Attribute Code',
    birth_date: 'Birth Date (YYYYMMDD)',
    gender: 'Gender',
    university: 'University',
    male: 'Male',
    female: 'Female',
    submit: 'Create',
    submitting: 'Creating...',
    no_data: 'No attribute codes available',
    birth_date_placeholder: 'e.g., 19900101',
    university_placeholder: 'e.g., Seoul National University',
    optional: '(Optional)',
    at_least_one: 'Please enter at least one attribute',
  },
  ko: {
    title: '속성 코드 관리',
    create_new: '새 코드 생성',
    loading: '로딩 중...',
    load_error: '로드 실패',
    code: '코드',
    created_at: '생성일',
    attributes: '속성',
    actions: '작업',
    delete: '삭제',
    delete_confirm: '이 속성 코드를 삭제하시겠습니까?',
    cancel: '취소',
    confirm: '확인',
    create_title: '속성 코드 생성',
    birth_date: '생년월일 (YYYYMMDD)',
    gender: '성별',
    university: '대학교',
    male: '남성',
    female: '여성',
    submit: '생성',
    submitting: '생성 중...',
    no_data: '속성 코드가 없습니다',
    birth_date_placeholder: '예: 19900101',
    university_placeholder: '예: 서울대학교',
    optional: '(선택)',
    at_least_one: '최소 하나의 속성을 입력해주세요',
  },
};

export interface AttributeCodesI18n {
  title: string;
  createNew: string;
  loading: string;
  loadError: string;
  code: string;
  createdAt: string;
  attributes: string;
  actions: string;
  delete: string;
  deleteConfirm: string;
  cancel: string;
  confirm: string;
  createTitle: string;
  birthDate: string;
  gender: string;
  university: string;
  male: string;
  female: string;
  submit: string;
  submitting: string;
  noData: string;
  birthDatePlaceholder: string;
  universityPlaceholder: string;
  optional: string;
  atLeastOne: string;
}

export function useAttributeCodesI18n(): AttributeCodesI18n {
  const { t } = useTranslation('AttributeCodes');

  return {
    title: t('title'),
    createNew: t('create_new'),
    loading: t('loading'),
    loadError: t('load_error'),
    code: t('code'),
    createdAt: t('created_at'),
    attributes: t('attributes'),
    actions: t('actions'),
    delete: t('delete'),
    deleteConfirm: t('delete_confirm'),
    cancel: t('cancel'),
    confirm: t('confirm'),
    createTitle: t('create_title'),
    birthDate: t('birth_date'),
    gender: t('gender'),
    university: t('university'),
    male: t('male'),
    female: t('female'),
    submit: t('submit'),
    submitting: t('submitting'),
    noData: t('no_data'),
    birthDatePlaceholder: t('birth_date_placeholder'),
    universityPlaceholder: t('university_placeholder'),
    optional: t('optional'),
    atLeastOne: t('at_least_one'),
  };
}
