import { useTranslation } from 'react-i18next';

export const i18nAdmins = {
  en: {
    title: 'Admin User Management',
    loading: 'Loading admin users...',
    load_error: 'Failed to load admin users',
    no_admins: 'No admin users found',
    promote_admin: 'Promote to Admin',
    demote_admin: 'Demote Admin',
    confirm_demote: 'Confirm Demote',
    cancel: 'Cancel',
    confirm: 'Confirm',
    email: 'Email',
    username: 'Username',
    display_name: 'Display Name',
    user_type: 'User Type',
    created_at: 'Created At',
    actions: 'Actions',
    promote_dialog_title: 'Promote User to Admin',
    promote_dialog_description:
      'Enter the email address of the user you want to promote to admin',
    demote_dialog_title: 'Demote Admin User',
    demote_dialog_description:
      'Are you sure you want to demote this user from admin?',
    demote_warning: 'This will remove all admin privileges from this user.',
    email_placeholder: 'user@example.com',
    promote_success: 'User promoted to admin successfully',
    demote_success: 'Admin demoted successfully',
    error_occurred: 'An error occurred',
  },
  ko: {
    title: '관리자 사용자 관리',
    loading: '관리자 사용자 로딩 중...',
    load_error: '관리자 사용자 로드 실패',
    no_admins: '관리자 사용자가 없습니다',
    promote_admin: '관리자로 승격',
    demote_admin: '관리자 강등',
    confirm_demote: '강등 확인',
    cancel: '취소',
    confirm: '확인',
    email: '이메일',
    username: '사용자명',
    display_name: '표시 이름',
    user_type: '사용자 유형',
    created_at: '생성일',
    actions: '작업',
    promote_dialog_title: '사용자를 관리자로 승격',
    promote_dialog_description:
      '관리자로 승격할 사용자의 이메일 주소를 입력하세요',
    demote_dialog_title: '관리자 강등',
    demote_dialog_description: '이 사용자를 관리자에서 강등하시겠습니까?',
    demote_warning: '이 작업은 이 사용자의 모든 관리자 권한을 제거합니다.',
    email_placeholder: 'user@example.com',
    promote_success: '사용자가 관리자로 승격되었습니다',
    demote_success: '관리자가 강등되었습니다',
    error_occurred: '오류가 발생했습니다',
  },
};

export interface AdminsI18n {
  title: string;
  loading: string;
  loadError: string;
  noAdmins: string;
  promoteAdmin: string;
  demoteAdmin: string;
  confirmDemote: string;
  cancel: string;
  confirm: string;
  email: string;
  username: string;
  displayName: string;
  userType: string;
  createdAt: string;
  actions: string;
  promoteDialogTitle: string;
  promoteDialogDescription: string;
  demoteDialogTitle: string;
  demoteDialogDescription: string;
  demoteWarning: string;
  emailPlaceholder: string;
  promoteSuccess: string;
  demoteSuccess: string;
  errorOccurred: string;
}

export function useAdminsI18n(): AdminsI18n {
  const { t } = useTranslation('Admins');

  return {
    title: t('title'),
    loading: t('loading'),
    loadError: t('load_error'),
    noAdmins: t('no_admins'),
    promoteAdmin: t('promote_admin'),
    demoteAdmin: t('demote_admin'),
    confirmDemote: t('confirm_demote'),
    cancel: t('cancel'),
    confirm: t('confirm'),
    email: t('email'),
    username: t('username'),
    displayName: t('display_name'),
    userType: t('user_type'),
    createdAt: t('created_at'),
    actions: t('actions'),
    promoteDialogTitle: t('promote_dialog_title'),
    promoteDialogDescription: t('promote_dialog_description'),
    demoteDialogTitle: t('demote_dialog_title'),
    demoteDialogDescription: t('demote_dialog_description'),
    demoteWarning: t('demote_warning'),
    emailPlaceholder: t('email_placeholder'),
    promoteSuccess: t('promote_success'),
    demoteSuccess: t('demote_success'),
    errorOccurred: t('error_occurred'),
  };
}
