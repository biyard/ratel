import { useTranslation } from 'react-i18next';

export const i18nSpaceHome = {
  en: {
    failedPdfUpload: 'Failed to PDF upload',
    onlyPdfFiles: 'Only PDF files can uploaded',
    fileSizeLimit: 'Each file must be less than 50MB.',
    failedIssueUploadUrl: 'Failed to issue upload URL.',
    completePdfUpload: 'Complete to PDF upload',
    successUpdateFiles: 'Success to update space files',
    failedUpdateFiles: 'Failed to update space files',
    successUpdateContent: 'Success to update space content',
    failedUpdateContent: 'Failed to update space content',
    successUpdateTitle: 'Success to update space title',
    failedUpdateTitle: 'Failed to update space title',
    failedUploadImage: 'Failed to upload image',
    change_private: 'Convert Private',
    change_public: 'Convert Public',
    publish_space: 'Publish Space',
    start_space: 'Start Space',
    end_space: 'End Space',
    delete_space: 'Delete Space',
    unsupported_space_type: 'Unsupported space type',
    no_authorized_user: 'No Authorized User',
    untitled_space: 'Untitled Space',
    start_warning:
      'This action cannot be undone. Starting a space will limit user participations.',
    end_warning:
      'This action cannot be undone. Ending a space will restrict user participations in space.',
    start_button: 'Start Space',
    starting: 'Starting...',
    end_button: 'Finish Space',
    ending: 'Finishing....',

    delete_title: 'Delete Space <name></name>',
    delete_warning:
      'This action cannot be undone. This will permanently delete the Space and all its contents.',
    delete_label: 'To confirm, type the Space name exactly as shown:',
    delete_placeholder: 'Type "{{spaceName}}" to confirm',
    cancel: 'Cancel',
    delete_button: 'Delete Space',
    deleting: 'Deleting...',
    go_public: 'Go Public',
    publish: 'Publish',
    save: 'Save',
    edit: 'Edit',
    make_public: 'Make Public',
    see_committee_list: 'See committee list',
    change_category: 'Change Category',
    delete: 'Delete',
    started: 'Start',
    finished: 'Finish',
    private: 'Private',
    public: 'Public',
    onboard: 'ONBOARD',
    input_title: 'Input title.',
    enable_anonymous_option_failed:
      'You should enable anonymous option before deliberation space publish.',
    make_public_title: 'You’re About to Go Public',
    make_public_desc_line1:
      'Once made public, this Space will be visible to everyone',
    make_public_desc_line2: 'and <b>cannot be made private again.</b>',

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
    menu_incentive_setting: 'Incentive Setting',
    menu_incentive: 'Incentive',
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

    // Actions
    action_participate: 'Participate',

    authorize_title: 'Failed to authorize attribute.',
    authorize_desc_1: 'The space lacks the required attributes.',
    authorize_desc_2:
      'Go to the credentials page, verify your credentials, and try accessing again.',
    go_credentials: 'Go to the credentials page',
    success_publish_space: 'Success to publish space.',
    failed_publish_space: 'Failed to publish space. please try later.',
    success_delete_space: 'Success to delete space.',
    failed_delete_space: 'Failed to delete space. please try later.',
    success_start_space: 'Success to start space.',
    failed_start_space: 'Failed to start space. please try later.',
    success_finish_space: 'Success to finish space.',
    failed_finish_space: 'Failed to finish space. please try later.',
    success_participate_space: 'Successfully joined the space.',
    failed_participate_space: 'Failed to join the space. please try later.',

    upload_media: 'Upload Media',
    uploading: 'Uploading…',
    upload_file_size_limit:
      'Videos can be uploaded up to a maximum size of 50MB.',

    end_space_title: 'Ended Space',
    end_space_desc: 'This space has ended.',
    go_home: 'Go to Home',
  },
  ko: {
    failedPdfUpload: 'PDF 업로드 실패',
    onlyPdfFiles: 'PDF 파일만 업로드 가능합니다.',
    fileSizeLimit: '파일 크기는 50MB 이하여야 합니다.',
    failedIssueUploadUrl: '업로드 URL 발급에 실패했습니다.',
    completePdfUpload: 'PDF 업로드 완료',
    successUpdateFiles: '스페이스 파일 업데이트 성공',
    failedUpdateFiles: '스페이스 파일 업데이트 실패',
    successUpdateContent: '스페이스 내용 업데이트 성공',
    failedUpdateContent: '스페이스 내용 업데이트 실패',
    successUpdateTitle: '스페이스 제목 업데이트 성공',
    failedUpdateTitle: '스페이스 제목 업데이트 실패',
    failedUploadImage: '이미지 업로드 실패',
  },
};

export interface I18nSpaceHome {
  failedPdfUpload: string;
  onlyPdfFiles: string;
  fileSizeLimit: string;
  failedIssueUploadUrl: string;
  completePdfUpload: string;
  successUpdateFiles: string;
  failedUpdateFiles: string;
  successUpdateContent: string;
  failedUpdateContent: string;
  successUpdateTitle: string;
  failedUpdateTitle: string;
  failedUploadImage: string;
}

export function useSpaceHomeI18n(): I18nSpaceHome {
  const { t } = useTranslation('SpaceHome');
  return {
    failedPdfUpload: t('failedPdfUpload'),
    onlyPdfFiles: t('onlyPdfFiles'),
    fileSizeLimit: t('fileSizeLimit'),
    failedIssueUploadUrl: t('failedIssueUploadUrl'),
    completePdfUpload: t('completePdfUpload'),
    successUpdateFiles: t('successUpdateFiles'),
    failedUpdateFiles: t('failedUpdateFiles'),
    successUpdateContent: t('successUpdateContent'),
    failedUpdateContent: t('failedUpdateContent'),
    successUpdateTitle: t('successUpdateTitle'),
    failedUpdateTitle: t('failedUpdateTitle'),
    failedUploadImage: t('failedUploadImage'),
  };
}
