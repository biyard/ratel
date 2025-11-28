import { useTranslation } from 'react-i18next';

export const i18nNotifications = {
  en: {
    title: 'Notifications',
    empty: 'No notifications',
    empty_description: 'You have no notifications at this time',
    mark_as_read: 'Mark as read',
    mark_all_as_read: 'Mark all as read',
    delete: 'Delete',
    delete_confirmation: 'Are you sure you want to delete this notification?',
    cancel: 'Cancel',
    confirm: 'Confirm',
    unread: 'Unread',
    read: 'Read',
    // Notification types
    space_post_notification: 'New post in space',
    team_invite: 'Team invitation',
    space_invite_verification: 'Space invitation',
    signup_security_code: 'Security code',
    start_survey: 'New survey',
    unknown: 'Notification',
  },
  ko: {
    title: '알림',
    empty: '알림 없음',
    empty_description: '현재 알림이 없습니다',
    mark_as_read: '읽음으로 표시',
    mark_all_as_read: '모두 읽음으로 표시',
    delete: '삭제',
    delete_confirmation: '이 알림을 삭제하시겠습니까?',
    cancel: '취소',
    confirm: '확인',
    unread: '읽지 않음',
    read: '읽음',
    // Notification types
    space_post_notification: '스페이스 새 게시물',
    team_invite: '팀 초대',
    space_invite_verification: '스페이스 초대',
    signup_security_code: '보안 코드',
    start_survey: '새 설문',
    unknown: '알림',
  },
};

export interface NotificationsI18n {
  title: string;
  empty: string;
  empty_description: string;
  mark_as_read: string;
  mark_all_as_read: string;
  delete: string;
  delete_confirmation: string;
  cancel: string;
  confirm: string;
  unread: string;
  read: string;
  space_post_notification: string;
  team_invite: string;
  space_invite_verification: string;
  signup_security_code: string;
  start_survey: string;
  unknown: string;
}

export function useNotificationsI18n(): NotificationsI18n {
  const { t } = useTranslation('Notifications');

  return {
    title: t('title'),
    empty: t('empty'),
    empty_description: t('empty_description'),
    mark_as_read: t('mark_as_read'),
    mark_all_as_read: t('mark_all_as_read'),
    delete: t('delete'),
    delete_confirmation: t('delete_confirmation'),
    cancel: t('cancel'),
    confirm: t('confirm'),
    unread: t('unread'),
    read: t('read'),
    space_post_notification: t('space_post_notification'),
    team_invite: t('team_invite'),
    space_invite_verification: t('space_invite_verification'),
    signup_security_code: t('signup_security_code'),
    start_survey: t('start_survey'),
    unknown: t('unknown'),
  };
}
