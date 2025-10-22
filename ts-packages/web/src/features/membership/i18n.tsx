import { useTranslation } from 'react-i18next';

export const i18nMemberships = {
  en: {
    title: 'Membership Management',
    create_new: 'Create New Membership',
    loading: 'Loading...',
    load_error: 'Error loading memberships',
    no_memberships: 'No memberships found',

    tier: 'Tier',
    price: 'Price',
    credits: 'Credits',
    duration: 'Duration',
    display_order: 'Display Order',
    status: 'Status',
    actions: 'Actions',
    days: 'days',
    active: 'Active',
    inactive: 'Inactive',

    edit: 'Edit',
    delete: 'Delete',
    submit: 'Submit',
    cancel: 'Cancel',
    submitting: 'Submitting...',
    deleting: 'Deleting...',

    create_membership: 'Create Membership',
    edit_membership: 'Edit Membership',
    is_active: 'Is Active',
    submit_error: 'Failed to submit form',

    delete_confirm_title: 'Delete Membership',
    delete_confirm_message:
      'Are you sure you want to delete the {tier} membership? This action cannot be undone.',
    delete_confirm: 'Delete',
  },
  ko: {
    title: '멤버십 관리',
    create_new: '새 멤버십 만들기',
    loading: '로딩 중...',
    load_error: '멤버십 로드 오류',
    no_memberships: '멤버십이 없습니다',

    tier: '등급',
    price: '가격',
    credits: '크레딧',
    duration: '기간',
    display_order: '표시 순서',
    status: '상태',
    actions: '작업',
    days: '일',
    active: '활성',
    inactive: '비활성',

    edit: '수정',
    delete: '삭제',
    submit: '제출',
    cancel: '취소',
    submitting: '제출 중...',
    deleting: '삭제 중...',

    create_membership: '멤버십 생성',
    edit_membership: '멤버십 수정',
    is_active: '활성화',
    submit_error: '양식 제출 실패',

    delete_confirm_title: '멤버십 삭제',
    delete_confirm_message:
      '{tier} 멤버십을 삭제하시겠습니까? 이 작업은 취소할 수 없습니다.',
    delete_confirm: '삭제',
  },
};

export interface MembershipsI18n {
  title: string;
  createNew: string;
  loading: string;
  loadError: string;
  noMemberships: string;

  // Table columns
  tier: string;
  price: string;
  credits: string;
  duration: string;
  displayOrder: string;
  status: string;
  actions: string;
  days: string;
  active: string;
  inactive: string;

  // Actions
  edit: string;
  delete: string;
  submit: string;
  cancel: string;
  submitting: string;
  deleting: string;

  // Form
  createMembership: string;
  editMembership: string;
  isActive: string;
  submitError: string;

  // Delete dialog
  deleteConfirmTitle: string;
  deleteConfirmMessage: string;
  deleteConfirm: string;
}

export function useMembershipsI18n(): MembershipsI18n {
  const { t } = useTranslation('Memberships');

  return {
    title: t('title'),
    createNew: t('create_new'),
    loading: t('loading'),
    loadError: t('load_error'),
    noMemberships: t('no_memberships'),

    tier: t('tier'),
    price: t('price'),
    credits: t('credits'),
    duration: t('duration'),
    displayOrder: t('display_order'),
    status: t('status'),
    actions: t('actions'),
    days: t('days'),
    active: t('active'),
    inactive: t('inactive'),

    edit: t('edit'),
    delete: t('delete'),
    submit: t('submit'),
    cancel: t('cancel'),
    submitting: t('submitting'),
    deleting: t('deleting'),

    createMembership: t('create_membership'),
    editMembership: t('edit_membership'),
    isActive: t('is_active'),
    submitError: t('submit_error'),

    deleteConfirmTitle: t('delete_confirm_title'),
    deleteConfirmMessage: t('delete_confirm_message'),
    deleteConfirm: t('delete_confirm'),
  };
}
