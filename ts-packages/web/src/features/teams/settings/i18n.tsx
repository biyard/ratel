import { useTranslation } from 'react-i18next';

export const i18nTeamSettingsPage = {
  en: {
    username: 'Username',
    display_name: 'Display Name',
    description: 'Description',
    dao_address: 'DAO Address',
    upload_logo: 'Upload Logo',
    display_name_hint: 'Enter team display name',
    team_description_hint: 'Enter team description',
    activate_dao: 'Activate DAO',
    activating_dao: 'Activating...',
    save: 'Save',
    delete: 'Delete',
    cancel: 'Cancel',
    confirm: 'Confirm',
    success_delete_team: 'Team deleted successfully',
    failed_delete_team: 'Failed to delete team',
    failed_update_team: 'Failed to update team',
    remove_test_keyword: 'Please remove test keywords',
    delete_team_title: 'Delete Team',
    delete_team_description:
      'Are you sure you want to delete this team? This action cannot be undone.',
  },
  ko: {
    username: '사용자명',
    display_name: '표시 이름',
    description: '설명',
    dao_address: 'DAO 주소',
    upload_logo: '로고 업로드',
    display_name_hint: '팀 표시 이름을 입력하세요',
    team_description_hint: '팀 설명을 입력하세요',
    activate_dao: 'DAO 활성화',
    activating_dao: '활성화 중...',
    save: '저장',
    delete: '삭제',
    cancel: '취소',
    confirm: '확인',
    success_delete_team: '팀이 성공적으로 삭제되었습니다',
    failed_delete_team: '팀 삭제에 실패했습니다',
    failed_update_team: '팀 업데이트에 실패했습니다',
    remove_test_keyword: '테스트 키워드를 제거해주세요',
    delete_team_title: '팀 삭제',
    delete_team_description:
      '정말로 이 팀을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.',
  },
};

export interface TeamSettingsI18n {
  username: string;
  display_name: string;
  description: string;
  dao_address: string;
  upload_logo: string;
  display_name_hint: string;
  team_description_hint: string;
  activate_dao: string;
  activating_dao: string;
  save: string;
  delete: string;
  cancel: string;
  confirm: string;
  success_delete_team: string;
  failed_delete_team: string;
  failed_update_team: string;
  remove_test_keyword: string;
  delete_team_title: string;
  delete_team_description: string;
}

export function useTeamSettingsI18n(): TeamSettingsI18n {
  const { t } = useTranslation('TeamSettings');
  return {
    username: t('username'),
    display_name: t('display_name'),
    description: t('description'),
    dao_address: t('dao_address'),
    upload_logo: t('upload_logo'),
    display_name_hint: t('display_name_hint'),
    team_description_hint: t('team_description_hint'),
    activate_dao: t('activate_dao'),
    activating_dao: t('activating_dao'),
    save: t('save'),
    delete: t('delete'),
    cancel: t('cancel'),
    confirm: t('confirm'),
    success_delete_team: t('success_delete_team'),
    failed_delete_team: t('failed_delete_team'),
    failed_update_team: t('failed_update_team'),
    remove_test_keyword: t('remove_test_keyword'),
    delete_team_title: t('delete_team_title'),
    delete_team_description: t('delete_team_description'),
  };
}
