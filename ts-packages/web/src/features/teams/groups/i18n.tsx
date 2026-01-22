import { useTranslation } from 'react-i18next';

export const i18nTeamGroups = {
  en: {
    // Permission group names
    permission_group_post: 'Post',
    permission_group_admin: 'Admin',

    // Post permissions
    permission_post_read: 'Read posts',
    permission_post_write: 'Write posts',
    permission_post_delete: 'Delete posts',

    // Admin permissions
    permission_group_edit: 'Edit Group',
    permission_team_edit: 'Edit Team',
    permission_team_admin: 'Admin',

    // Form labels
    group_name: 'Group Name',
    group_name_hint: 'Enter your group name',
    description: 'Description',
    description_hint: 'What is the purpose of this group?',
    permission: 'Permissions',
    select_all: 'Select All',

    // Buttons
    create: 'Create',

    // Error messages
    group_name_required: 'Group name is required',
    group_image_required: 'Group image is required',
    group_option_required: 'Please select at least one permission',
  },
  ko: {
    // Permission group names
    permission_group_post: '게시물',
    permission_group_admin: '관리자',

    // Post permissions
    permission_post_read: '게시물 읽기',
    permission_post_write: '게시물 작성',
    permission_post_delete: '게시물 삭제',

    // Admin permissions
    permission_group_edit: '그룹 편집',
    permission_team_edit: '팀 편집',
    permission_team_admin: '관리자',

    // Form labels
    group_name: '그룹 이름',
    group_name_hint: '그룹 이름을 입력하세요',
    description: '설명',
    description_hint: '그룹의 목적은 무엇인가요?',
    permission: '권한',
    select_all: '전체 선택',

    // Buttons
    create: '생성',

    // Error messages
    group_name_required: '그룹 이름을 입력해주세요',
    group_image_required: '그룹 이미지를 선택해주세요',
    group_option_required: '적어도 하나의 권한을 선택해주세요',
  },
};

export interface TeamGroupsI18n {
  // Permission group names
  permission_group_post: string;
  permission_group_admin: string;

  // Post permissions
  permission_post_read: string;
  permission_post_write: string;
  permission_post_delete: string;

  // Admin permissions
  permission_group_edit: string;
  permission_team_edit: string;
  permission_team_admin: string;

  // Form labels
  group_name: string;
  group_name_hint: string;
  description: string;
  description_hint: string;
  permission: string;
  select_all: string;

  // Buttons
  create: string;

  // Error messages
  group_name_required: string;
  group_image_required: string;
  group_option_required: string;
}

export function useTeamGroupsI18n(): TeamGroupsI18n {
  const { t } = useTranslation('TeamGroups');
  return {
    // Permission group names
    permission_group_post: t('permission_group_post'),
    permission_group_admin: t('permission_group_admin'),

    // Post permissions
    permission_post_read: t('permission_post_read'),
    permission_post_write: t('permission_post_write'),
    permission_post_delete: t('permission_post_delete'),

    // Admin permissions
    permission_group_edit: t('permission_group_edit'),
    permission_team_edit: t('permission_team_edit'),
    permission_team_admin: t('permission_team_admin'),

    // Form labels
    group_name: t('group_name'),
    group_name_hint: t('group_name_hint'),
    description: t('description'),
    description_hint: t('description_hint'),
    permission: t('permission'),
    select_all: t('select_all'),

    // Buttons
    create: t('create'),

    // Error messages
    group_name_required: t('group_name_required'),
    group_image_required: t('group_image_required'),
    group_option_required: t('group_option_required'),
  };
}
