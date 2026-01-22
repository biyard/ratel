import { lang } from '@/i18n/config';
import { useTranslation } from 'react-i18next';

export const i18nSpaceRewards: Record<lang, SpaceRewardsI18n> = {
  en: {
    sidemenu: {
      title: 'Reward',
      totalPoint: 'Total Point',
    },
    settings: {
      title: 'Reward Settings',
      loading: 'Loading...',
      error: 'Failed to load rewards',

      no_polls: 'No polls available',
      no_polls_description:
        'Create a poll first to set up rewards for poll responses.',
      no_rewards: 'No rewards configured',
      no_rewards_description: 'Click the button below to add a reward.',

      reward_label: 'Reward',
      credits: 'Credits',
      total_claims: 'Total Claims',
      total_points: 'Total Points',

      description: 'Description',
      description_placeholder: 'Enter reward description (optional)',
      credits_placeholder: 'Enter credits amount',
      credits_required: 'Credits amount is required',
      credits_min: 'Credits must be at least 1',

      create_reward: 'Add Reward',
      edit_reward: 'Edit Reward',
      delete_reward: 'Delete Reward',
      save: 'Save',
      cancel: 'Cancel',

      create_success: 'Reward created successfully',
      create_error: 'Failed to create reward',
      update_success: 'Reward updated successfully',
      update_error: 'Failed to update reward',
      delete_success: 'Reward deleted successfully',
      delete_error: 'Failed to delete reward',

      delete_confirm_title: 'Delete Reward',
      delete_confirm_message:
        'Are you sure you want to delete this reward? This action cannot be undone.',

      poll_reward_section: 'Poll Rewards',
      poll_respond: 'Poll Response Reward',
      poll_respond_reward: 'Poll Response Reward',
      board_comment_reward: 'Board Comment Reward',
      board_like_reward: 'Board Like Reward',
      unknown_reward: 'Reward',

      reward_action: 'Reward Type',
      reward_type_required: 'Please select a reward type',
      select_reward_type: 'Select reward type',
      points: 'Reward Points',
    },
  },
  ko: {
    sidemenu: {
      title: '리워드',
      totalPoint: '총 포인트',
    },
    settings: {
      title: '리워드 설정',
      loading: '로딩 중...',
      error: '리워드를 불러오는데 실패했습니다',

      no_polls: '설문이 없습니다',
      no_polls_description:
        '설문 응답 리워드를 설정하려면 먼저 설문을 생성해주세요.',
      no_rewards: '설정된 리워드가 없습니다',
      no_rewards_description: '아래 버튼을 클릭하여 리워드를 추가하세요.',

      reward_label: '리워드',
      credits: '크레딧',
      total_claims: '총 청구 횟수',
      total_points: '총 포인트',

      description: '설명',
      description_placeholder: '리워드 설명을 입력하세요 (선택사항)',
      credits_placeholder: '크레딧 금액을 입력하세요',
      credits_required: '크레딧 금액은 필수입니다',
      credits_min: '크레딧은 최소 1 이상이어야 합니다',

      create_reward: '리워드 추가',
      edit_reward: '리워드 수정',
      delete_reward: '리워드 삭제',
      save: '저장',
      cancel: '취소',

      create_success: '리워드가 성공적으로 생성되었습니다',
      create_error: '리워드 생성에 실패했습니다',
      update_success: '리워드가 성공적으로 업데이트되었습니다',
      update_error: '리워드 업데이트에 실패했습니다',
      delete_success: '리워드가 성공적으로 삭제되었습니다',
      delete_error: '리워드 삭제에 실패했습니다',

      delete_confirm_title: '리워드 삭제',
      delete_confirm_message:
        '이 리워드를 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.',

      poll_reward_section: '설문 리워드',
      poll_respond: '설문 응답 리워드',
      poll_respond_reward: '설문 응답 리워드',
      board_comment_reward: '게시판 댓글 리워드',
      board_like_reward: '게시판 좋아요 리워드',

      unknown_reward: '리워드',

      reward_action: '리워드 타입',
      reward_type_required: '리워드 타입을 선택해주세요',
      select_reward_type: '리워드 타입 선택',
      points: '기본 포인트',
    },
  },
};

export interface SpaceRewardsI18n {
  sidemenu: SideMenu;
  settings: SettingsI18n;
}

interface SideMenu {
  title: string;
  totalPoint: string;
}

interface SettingsI18n {
  title: string;
  loading: string;
  error: string;

  no_polls: string;
  no_polls_description: string;
  no_rewards: string;
  no_rewards_description: string;

  reward_label: string;
  credits: string;
  total_claims: string;
  total_points: string;

  description: string;
  description_placeholder: string;
  credits_placeholder: string;
  credits_required: string;
  credits_min: string;

  create_reward: string;
  edit_reward: string;
  delete_reward: string;
  save: string;
  cancel: string;

  create_success: string;
  create_error: string;
  update_success: string;
  update_error: string;
  delete_success: string;
  delete_error: string;

  delete_confirm_title: string;
  delete_confirm_message: string;

  poll_reward_section: string;
  poll_respond: string;
  poll_respond_reward: string;
  board_comment_reward: string;
  board_like_reward: string;
  unknown_reward: string;

  reward_action: string;
  reward_type_required: string;
  select_reward_type: string;
  points: string;
}

export function useSpaceRewardsI18n(): SpaceRewardsI18n {
  const { t } = useTranslation('SpaceRewards');
  return {
    sidemenu: {
      title: t('sidemenu.title'),
      totalPoint: t('sidemenu.totalPoint'),
    },
    settings: {
      title: t('settings.title'),
      loading: t('settings.loading'),
      error: t('settings.error'),

      no_polls: t('settings.no_polls'),
      no_polls_description: t('settings.no_polls_description'),
      no_rewards: t('settings.no_rewards'),
      no_rewards_description: t('settings.no_rewards_description'),

      reward_label: t('settings.reward_label'),
      credits: t('settings.credits'),
      total_claims: t('settings.total_claims'),
      total_points: t('settings.total_points'),

      description: t('settings.description'),
      description_placeholder: t('settings.description_placeholder'),
      credits_placeholder: t('settings.credits_placeholder'),
      credits_required: t('settings.credits_required'),
      credits_min: t('settings.credits_min'),

      create_reward: t('settings.create_reward'),
      edit_reward: t('settings.edit_reward'),
      delete_reward: t('settings.delete_reward'),
      save: t('settings.save'),
      cancel: t('settings.cancel'),

      create_success: t('settings.create_success'),
      create_error: t('settings.create_error'),
      update_success: t('settings.update_success'),
      update_error: t('settings.update_error'),
      delete_success: t('settings.delete_success'),
      delete_error: t('settings.delete_error'),

      delete_confirm_title: t('settings.delete_confirm_title'),
      delete_confirm_message: t('settings.delete_confirm_message'),

      poll_reward_section: t('settings.poll_reward_section'),
      poll_respond: t('settings.poll_respond'),
      poll_respond_reward: t('settings.poll_respond_reward'),
      board_comment_reward: t('settings.board_comment_reward'),
      board_like_reward: t('settings.board_like_reward'),
      unknown_reward: t('settings.unknown_reward'),

      reward_action: t('settings.reward_action'),
      reward_type_required: t('settings.reward_type_required'),
      select_reward_type: t('settings.select_reward_type'),
      points: t('settings.points'),
    },
  };
}
