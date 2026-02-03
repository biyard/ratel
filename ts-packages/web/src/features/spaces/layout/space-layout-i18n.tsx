import { useTranslation } from 'react-i18next';
import { Label } from './space-menus';

export interface I18nSpaceLayout extends Record<Label, string> {
  //Admin Card
  admin_title: string;
  admin_description: string;

  // Candidate Card
  candidate_title: string;
  candidate_description: string;
  candidate_participant_button_label: string;
  candidate_credentials_button_label: string;

  // Timeline
  timeline_title: string;
  timeline_created_at_label: string;

  // Side Menu
  menu_poll: string;
  menu_discussions: string;
  menu_panels: string;
  menu_boards: string;
  menu_members: string;
  menu_files: string;
  menu_dao: string;
  menu_quiz: string;
  menu_recommendations: string;
  menu_sprint_league: string;
  menu_nft_preview: string;
  menu_nft_settings: string;
  menu_nft_art_twin: string;

  // Admin Menus
  menu_admin_settings: string;
  menu_rewards: string;
  menu_analyze: string;

  // Admin Action
  action_admin_start: string;
  action_admin_delete: string;
  action_admin_publish: string;

  // Candidate Action
  action_candidate_participate: string;
  action_candidate_credentials: string;

  // Toast Message
  toast_participate_success: string;
  toast_participate_failed: string;
  toast_start_success: string;
  toast_start_failed: string;
  toast_delete_success: string;
  toast_delete_failed: string;
  toast_publish_success: string;
  toast_publish_failed: string;
  toast_update_title_success: string;
  toast_update_title_failed: string;

  // Modal Titles
  publish_space_title: string;
  delete_space_title: string;
  start_space_title: string;
}

export const i18nSpaceLayout = {
  en: {
    // Timeline
    timeline_title: 'Timeline',
    timeline_created_at_label: 'Created',

    // Menus
    menu_overview: 'Overview',
    menu_poll: 'Polls',
    menu_discussions: 'Discussions',
    menu_panels: 'Panels',
    menu_boards: 'Boards',
    menu_members: 'Members',
    menu_files: 'Files',
    menu_dao: 'DAO',
    menu_quiz: 'Quizzes',
    menu_recommendations: 'Recommendations',
    menu_sprint_league: 'Sprint League',
    // Admin Menus
    menu_admin_settings: 'Settings',
    menu_rewards: 'Rewards',
    menu_analyze: 'Analyze',
    menu_nft_preview: 'ArtNFT',
    menu_nft_settings: 'Settings',
    menu_nft_art_twin: 'Art Twin',

    // Admin Card
    admin_title: 'Admin',
    admin_description: 'You have full access to manage this space.',

    // Candidate
    candidate_title: 'Candidate',
    candidate_description:
      'You can read everything, but posting, voting and commenting require verification.',
    candidate_participant_button_label: 'Participate',
    candidate_credentials_button_label: 'See My Credential',

    // Admin Action
    action_admin_start: 'Start',
    action_admin_delete: 'Delete',
    action_admin_publish: 'Publish',
    action_candidate_participate: 'Participate',
    action_candidate_credentials: 'See Credentials',

    // Toast Message
    toast_participate_success: 'Successfully joined the space.',
    toast_participate_failed: 'Failed to join the space.',
    toast_start_success: 'Success to start space.',
    toast_start_failed: 'Failed to start space.',
    toast_delete_success: 'Success to delete space.',
    toast_delete_failed: 'Failed to delete space.',
    toast_publish_success: 'Success to publish space.',
    toast_publish_failed: 'Failed to publish space.',
    toast_update_title_success: 'Success to update space title.',
    toast_update_title_failed: 'Failed to update space title.',
    publish_space_title: 'Publish Space',
    delete_space_title: 'Delete Space',
    start_space_title: 'Start Space',
  },
  ko: {
    timeline_title: '타임라인',
    timeline_created_at_label: '생성됨',

    // Menus
    menu_overview: '개요',
    menu_poll: '설문',
    menu_discussions: '토론',
    menu_panels: '패널',
    menu_boards: '게시판',
    menu_members: '멤버',
    menu_files: '파일',
    menu_dao: '다오',
    menu_quiz: '퀴즈',
    menu_recommendations: '권고사항',

    menu_sprint_league: 'Sprint League',

    // Admin Menus
    menu_admin_settings: '설정',
    menu_rewards: '보상 설정',
    menu_analyze: '분석',
    menu_nft_preview: 'ArtNFT',
    menu_nft_settings: '설정',
    menu_nft_art_twin: 'Art Twin',

    // Admin Card
    admin_title: '관리자',
    admin_description: '모든 권한을 가지고 있습니다.',

    // Candidate
    candidate_title: '참여자',
    candidate_description:
      '모든 게시물을 읽을 수 있지만, 게시, 투표, 댓글을 작성하려면 인증이 필요합니다.',
    candidate_participant_button_label: '참여하기',
    candidate_credentials_button_label: '내 자격 증명 확인',

    // Admin Action
    action_admin_start: '시작하기',
    action_admin_delete: '삭제하기',
    action_admin_publish: '게시하기',
    action_candidate_participate: '참여하기',
    action_candidate_credentials: '자격 증명 확인',

    // Toast Message
    toast_participate_success: '스페이스에 성공적으로 참여했습니다.',
    toast_participate_failed: '스페이스 참여에 실패했습니다.',
    toast_start_success: '스페이스를 성공적으로 시작했습니다.',
    toast_start_failed: '스페이스 시작에 실패했습니다.',
    toast_delete_success: '스페이스를 성공적으로 삭제했습니다.',
    toast_delete_failed: '스페이스 삭제에 실패했습니다.',
    toast_publish_success: '스페이스를 성공적으로 게시했습니다.',
    toast_publish_failed: '스페이스 게시에 실패했습니다.',
    toast_update_title_success: '스페이스 제목을 성공적으로 변경했습니다.',
    toast_update_title_failed: '스페이스 제목 변경에 실패했습니다.',
    publish_space_title: '스페이스 게시',
    delete_space_title: '스페이스 삭제',
    start_space_title: '스페이스 시작',
  },
};

export function useSpaceLayoutI18n(): I18nSpaceLayout {
  const { t } = useTranslation('SpaceLayout');

  return {
    timeline_title: t('timeline_title'),
    timeline_created_at_label: t('timeline_created_at_label'),
    menu_overview: t('menu_overview'),
    menu_poll: t('menu_poll'),
    menu_discussions: t('menu_discussions'),
    menu_panels: t('menu_panels'),
    menu_boards: t('menu_boards'),
    menu_members: t('menu_members'),
    menu_files: t('menu_files'),
    menu_dao: t('menu_dao'),
    menu_quiz: t('menu_quiz'),
    menu_recommendations: t('menu_recommendations'),
    menu_sprint_league: t('menu_sprint_league'),
    menu_admin_settings: t('menu_admin_settings'),
    menu_rewards: t('menu_rewards'),
    menu_analyze: t('menu_analyze'),
    menu_nft_preview: t('menu_nft_preview'),
    menu_nft_settings: t('menu_nft_settings'),
    menu_nft_art_twin: t('menu_nft_art_twin'),

    admin_title: t('admin_title'),
    admin_description: t('admin_description'),

    candidate_title: t('candidate_title'),
    candidate_description: t('candidate_description'),
    candidate_participant_button_label: t('candidate_participant_button_label'),
    candidate_credentials_button_label: t('candidate_credentials_button_label'),

    action_admin_start: t('action_admin_start'),
    action_admin_delete: t('action_admin_delete'),
    action_admin_publish: t('action_admin_publish'),
    action_candidate_participate: t('action_candidate_participate'),
    action_candidate_credentials: t('action_candidate_credentials'),

    toast_participate_success: t('toast_participate_success'),
    toast_participate_failed: t('toast_participate_failed'),
    toast_start_success: t('toast_start_success'),
    toast_start_failed: t('toast_start_failed'),
    toast_delete_success: t('toast_delete_success'),
    toast_delete_failed: t('toast_delete_failed'),
    toast_publish_success: t('toast_publish_success'),
    toast_publish_failed: t('toast_publish_failed'),
    toast_update_title_success: t('toast_update_title_success'),
    toast_update_title_failed: t('toast_update_title_failed'),
    publish_space_title: t('publish_space_title'),
    delete_space_title: t('delete_space_title'),
    start_space_title: t('start_space_title'),
  };
}
