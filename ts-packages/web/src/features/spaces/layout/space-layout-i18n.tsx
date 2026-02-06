import { useTranslation } from 'react-i18next';
import { Label } from './space-menus';

export interface I18nSpaceLayout extends Record<Label, string> {
  // Common
  cancel: string;

  //Admin Card
  admin_title: string;
  admin_description: string;

  // Viewer Card
  viewer_title: string;
  viewer_description: string;
  viewer_participant_button_label: string;
  viewer_credentials_button_label: string;

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
  menu_incentive: string;
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
  action_admin_finish: string;
  action_admin_delete: string;
  action_admin_publish: string;

  // Candidate Action
  action_viewer_participate: string;
  action_viewer_credentials: string;

  // Toast Message
  toast_participate_success: string;
  toast_participate_failed: string;
  toast_start_success: string;
  toast_start_failed: string;
  toast_delete_success: string;
  toast_delete_failed: string;
  toast_finish_success: string;
  toast_finish_failed: string;
  toast_publish_success: string;
  toast_publish_failed: string;
  toast_update_title_success: string;
  toast_update_title_failed: string;

  // Space Authorize Modal
  authorize_modal_title: string;
  authorize_modal_desc_1: string;
  authorize_modal_desc_2: string;
  authorize_modal_go_credentials: string;

  // Space Publish Modal
  publish_modal_title: string;
  publish_modal_private: string;
  publish_modal_private_desc: string;
  publish_modal_public: string;
  publish_modal_public_desc: string;
  publish_modal_button_publish: string;

  // Space Delete Modal
  delete_modal_title: string;
  delete_modal_delete_warning: string;
  delete_modal_delete_label: string;
  delete_modal_delete_placeholder: string;
  delete_modal_button_delete: string;
  delete_modal_button_deleting: string;

  // Space End Modal
  end_modal_title: string;
  end_modal_desc: string;
  end_modal_button_end: string;
  end_modal_button_ending: string;

  // Space Start Modal
  start_modal_title: string;
  start_modal_warning: string;
  start_modal_button_start: string;
  start_modal_button_starting: string;

  // Profile Dropdown Menu
  profile_dark_theme: string;
  profile_sign_out: string;

  login: string;
}

export const i18nSpaceLayout = {
  en: {
    cancel: 'Cancel',
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
    menu_incentive: 'Incentives',
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
    menu_requirements: 'Requirements',

    // Admin Card
    admin_title: 'Admin',
    admin_description: 'You have full access to manage this space.',

    // Viewer
    viewer_title: 'Viewer',
    viewer_description:
      'You can read everything, but posting, voting and commenting require verification.',
    viewer_participant_button_label: 'Participate',
    viewer_credentials_button_label: 'See My Credential',

    // Admin Action
    action_admin_start: 'Start',
    action_admin_finish: 'Finish',
    action_admin_delete: 'Delete',
    action_admin_publish: 'Publish',
    action_viewer_participate: 'Participate',
    action_viewer_credentials: 'See Credentials',

    // Toast Message
    toast_participate_success: 'Successfully joined the space.',
    toast_participate_failed: 'Failed to join the space.',
    toast_start_success: 'Success to start space.',
    toast_start_failed: 'Failed to start space.',
    toast_delete_success: 'Success to delete space.',
    toast_delete_failed: 'Failed to delete space.',
    toast_finish_success: 'Success to finish space.',
    toast_finish_failed: 'Failed to finish space.',
    toast_publish_success: 'Success to publish space.',
    toast_publish_failed: 'Failed to publish space.',
    toast_update_title_success: 'Success to update space title.',
    toast_update_title_failed: 'Failed to update space title.',

    // Space Authorize Modal
    authorize_modal_title: 'Authorization Failed',
    authorize_modal_desc_1:
      'You do not meet the required credentials for this space.',
    authorize_modal_desc_2:
      'Please verify your credentials on the credentials page.',
    authorize_modal_go_credentials: 'Go to Credentials Page',

    // Space Publish Modal
    publish_modal_title: 'Publish Space',
    publish_modal_private: 'Private Publish',
    publish_modal_private_desc:
      'Only your team members will be able to access this space.',
    publish_modal_public: 'Public Publish',
    publish_modal_public_desc:
      'Anyone can access and participate in this space.',
    publish_modal_button_publish: 'Publish Space',

    // Space Delete Modal
    delete_modal_title: 'Delete Space',
    delete_modal_delete_warning:
      'This action cannot be undone. This will permanently delete the Space and all its contents.',
    delete_modal_delete_label:
      'To confirm, please type the space name exactly as shown below:',
    delete_modal_delete_placeholder: 'Type "{{spaceName}}" to confirm',
    delete_modal_button_delete: 'Delete Space',
    delete_modal_button_deleting: 'Deleting...',

    // Space End Modal
    end_modal_title: 'End Space',
    end_modal_desc: 'Are you sure you want to end this space?',
    end_modal_button_end: 'End Space',
    end_modal_button_ending: 'Ending...',

    // Space Start Modal
    start_modal_title: 'Start Space',
    start_modal_warning:
      'Once started, participants can begin their activities.',
    start_modal_button_start: 'Start Space',
    start_modal_button_starting: 'Starting...',

    // Profile Dropdown Menu
    profile_dark_theme: 'Dark Theme',
    profile_sign_out: 'Sign Out',

    login: 'Login',
  },
  ko: {
    cancel: '취소',
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
    menu_dao: 'DAO',
    menu_incentive: '인센티브',
    menu_quiz: '퀴즈',
    menu_recommendations: '권고사항',
    menu_requirements: '사전 조건',
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

    // Viewer
    viewer_title: 'Viewer',
    viewer_description:
      '모든 게시물을 읽을 수 있지만, 게시, 투표, 댓글을 작성하려면 인증이 필요합니다.',
    viewer_participant_button_label: '참여하기',
    viewer_credentials_button_label: '내 자격 증명 확인',

    // Admin Action
    action_admin_start: '시작하기',
    action_admin_finish: '종료하기',
    action_admin_delete: '삭제하기',
    action_admin_publish: '게시하기',
    action_viewer_participate: '참여하기',
    action_viewer_credentials: '자격 증명 확인',

    // Toast Message
    toast_participate_success: '스페이스에 성공적으로 참여했습니다.',
    toast_participate_failed: '스페이스 참여에 실패했습니다.',
    toast_start_success: '스페이스를 성공적으로 시작했습니다.',
    toast_start_failed: '스페이스 시작에 실패했습니다.',
    toast_delete_success: '스페이스를 성공적으로 삭제했습니다.',
    toast_delete_failed: '스페이스 삭제에 실패했습니다.',
    toast_finish_success: '스페이스를 성공적으로 종료했습니다.',
    toast_finish_failed: '스페이스 종료에 실패했습니다.',
    toast_publish_success: '스페이스를 성공적으로 게시했습니다.',
    toast_publish_failed: '스페이스 게시에 실패했습니다.',
    toast_update_title_success: '스페이스 제목을 성공적으로 변경했습니다.',
    toast_update_title_failed: '스페이스 제목 변경에 실패했습니다.',

    // Space Authorize Modal
    authorize_modal_title: '속성 검증 실패',
    authorize_modal_desc_1: '스페이스가 요구하는 속성이 부족합니다.',
    authorize_modal_desc_2: '자격 증명 페이지에서 검증을 수행해주세요.',
    authorize_modal_go_credentials: '자격 증명 페이지로 이동하기',

    // Space Publish Modal
    publish_modal_title: '스페이스 게시',
    publish_modal_private: '비공개 게시',
    publish_modal_private_desc:
      '초대받은 사용자만 이 스페이스에 접근할 수 있습니다.',
    publish_modal_public: '공개 게시',
    publish_modal_public_desc:
      '누구나 이 스페이스에 접근하고 참여할 수 있습니다.',
    publish_modal_button_publish: '게시하기',

    // Space Delete Modal
    delete_modal_title: '스페이스 삭제',
    delete_modal_delete_warning:
      '이 작업은 되돌릴 수 없습니다. 스페이스와 그 안의 모든 콘텐츠가 영구적으로 삭제됩니다.',
    delete_modal_delete_label:
      '확인을 위해 아래와 동일하게 스페이스 이름을 입력하세요:',
    delete_modal_delete_placeholder:
      '확인을 위해 "{{spaceName}}"을(를) 입력하세요',
    delete_modal_button_delete: '삭제하기',
    delete_modal_button_deleting: '삭제 중...',

    // Space End Modal
    end_modal_title: '스페이스 종료',
    end_modal_desc: '정말로 이 스페이스를 종료하시겠습니까?',
    end_modal_button_end: '종료하기',
    end_modal_button_ending: '종료 중...',

    // Space Start Modal
    start_modal_title: '스페이스 시작',
    start_modal_warning:
      '스페이스를 시작하면 참여자들이 활동을 시작할 수 있습니다.',
    start_modal_button_start: '시작하기',
    start_modal_button_starting: '시작 중...',

    // Profile Dropdown Menu
    profile_dark_theme: '다크 테마',
    profile_sign_out: '로그아웃',

    login: '로그인',
  },
};

export function useSpaceLayoutI18n() {
  const { t } = useTranslation('SpaceLayout');

  return {
    cancel: t('cancel'),
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
    menu_incentive: t('menu_incentive'),
    menu_quiz: t('menu_quiz'),
    menu_recommendations: t('menu_recommendations'),
    menu_sprint_league: t('menu_sprint_league'),
    menu_admin_settings: t('menu_admin_settings'),
    menu_rewards: t('menu_rewards'),
    menu_analyze: t('menu_analyze'),
    menu_nft_preview: t('menu_nft_preview'),
    menu_nft_settings: t('menu_nft_settings'),
    menu_nft_art_twin: t('menu_nft_art_twin'),
    menu_requirements: t('menu_requirements'),

    admin_title: t('admin_title'),
    admin_description: t('admin_description'),

    viewer_title: t('viewer_title'),
    viewer_description: t('viewer_description'),
    viewer_participant_button_label: t('viewer_participant_button_label'),
    viewer_credentials_button_label: t('viewer_credentials_button_label'),

    action_admin_start: t('action_admin_start'),
    action_admin_finish: t('action_admin_finish'),
    action_admin_delete: t('action_admin_delete'),
    action_admin_publish: t('action_admin_publish'),
    action_viewer_participate: t('action_viewer_participate'),
    action_viewer_credentials: t('action_viewer_credentials'),

    toast_participate_success: t('toast_participate_success'),
    toast_participate_failed: t('toast_participate_failed'),
    toast_start_success: t('toast_start_success'),
    toast_start_failed: t('toast_start_failed'),
    toast_delete_success: t('toast_delete_success'),
    toast_delete_failed: t('toast_delete_failed'),
    toast_finish_success: t('toast_finish_success'),
    toast_finish_failed: t('toast_finish_failed'),
    toast_publish_success: t('toast_publish_success'),
    toast_publish_failed: t('toast_publish_failed'),
    toast_update_title_success: t('toast_update_title_success'),
    toast_update_title_failed: t('toast_update_title_failed'),

    authorize_modal_title: t('authorize_modal_title'),
    authorize_modal_desc_1: t('authorize_modal_desc_1'),
    authorize_modal_desc_2: t('authorize_modal_desc_2'),
    authorize_modal_go_credentials: t('authorize_modal_go_credentials'),

    publish_modal_title: t('publish_modal_title'),
    publish_modal_private: t('publish_modal_private'),
    publish_modal_private_desc: t('publish_modal_private_desc'),
    publish_modal_public: t('publish_modal_public'),
    publish_modal_public_desc: t('publish_modal_public_desc'),
    publish_modal_button_publish: t('publish_modal_button_publish'),

    delete_modal_title: t('delete_modal_title'),
    delete_modal_delete_warning: t('delete_modal_delete_warning'),
    delete_modal_delete_label: t('delete_modal_delete_label'),
    delete_modal_delete_placeholder: (spaceName: string) =>
      t('delete_modal_delete_placeholder', { spaceName }),
    delete_modal_button_delete: t('delete_modal_button_delete'),
    delete_modal_button_deleting: t('delete_modal_button_deleting'),

    end_modal_title: t('end_modal_title'),
    end_modal_desc: t('end_modal_desc'),
    end_modal_button_end: t('end_modal_button_end'),
    end_modal_button_ending: t('end_modal_button_ending'),

    start_modal_title: t('start_modal_title'),
    start_modal_warning: t('start_modal_warning'),
    start_modal_button_start: t('start_modal_button_start'),
    start_modal_button_starting: t('start_modal_button_starting'),

    profile_dark_theme: t('profile_dark_theme'),
    profile_sign_out: t('profile_sign_out'),
    login: t('login'),
  };
}
