import { useTranslation } from 'react-i18next';

export const CreatePostPage = {
  en: {
    page_title: 'Create post',
    title_placeholder: 'Title',
    content_placeholder: 'Type your script',
    create_space_title: 'Create a Space',
    space_name_placeholder: 'Space name',
    space_description_placeholder: 'Description',
    skip_creating_space: 'Skip creating a space',
    publish: 'Publish',
    publishing: 'Publishing...',
    saving: 'Saving...',
    last_saved_at: 'Last saved at',
    remove_image: 'Remove image',

    // Success messages
    success_publish: 'Post published successfully!',
    success_save: 'Draft saved successfully',

    // Error messages
    error_init: 'Failed to initialize post',
    error_save: 'Auto-save failed',
    error_upload: 'Failed to upload image',
    error_publish: 'Failed to publish post',
    error_empty_fields: 'Please fill in both title and content',
  },
  ko: {
    page_title: '게시물 작성',
    title_placeholder: '제목',
    content_placeholder: '내용을 입력하세요',
    create_space_title: '스페이스 만들기',
    space_name_placeholder: '스페이스 이름',
    space_description_placeholder: '설명',
    skip_creating_space: '스페이스 만들기 건너뛰기',
    publish: '게시',
    publishing: '게시 중...',
    saving: '저장 중...',
    last_saved_at: '마지막 저장',
    remove_image: '이미지 제거',

    // Success messages
    success_publish: '게시물이 성공적으로 게시되었습니다!',
    success_save: '임시 저장되었습니다',

    // Error messages
    error_init: '게시물 초기화에 실패했습니다',
    error_save: '자동 저장에 실패했습니다',
    error_upload: '이미지 업로드에 실패했습니다',
    error_publish: '게시물 게시에 실패했습니다',
    error_empty_fields: '제목과 내용을 모두 입력해주세요',
  },
};

export interface I18nCreatePostPage {
  page_title: string;
  title_placeholder: string;
  content_placeholder: string;
  create_space_title: string;
  space_name_placeholder: string;
  space_description_placeholder: string;
  skip_creating_space: string;
  publish: string;
  publishing: string;
  saving: string;
  last_saved_at: string;
  remove_image: string;

  // Success messages
  success_publish: string;
  success_save: string;

  // Error messages
  error_init: string;
  error_save: string;
  error_upload: string;
  error_publish: string;
  error_empty_fields: string;
}

export function useCreatePostPageI18n() {
  const { t } = useTranslation();

  return {
    page_title: t('CreatePostPage:page_title'),
    title_placeholder: t('CreatePostPage:title_placeholder'),
    content_placeholder: t('CreatePostPage:content_placeholder'),
    create_space_title: t('CreatePostPage:create_space_title'),
    space_name_placeholder: t('CreatePostPage:space_name_placeholder'),
    space_description_placeholder: t(
      'CreatePostPage:space_description_placeholder',
    ),
    skip_creating_space: t('CreatePostPage:skip_creating_space'),
    publish: t('CreatePostPage:publish'),
    publishing: t('CreatePostPage:publishing'),
    saving: t('CreatePostPage:saving'),
    last_saved_at: t('CreatePostPage:last_saved_at'),
    remove_image: t('CreatePostPage:remove_image'),

    // Success messages
    success_publish: t('CreatePostPage:success_publish'),
    success_save: t('CreatePostPage:success_save'),

    // Error messages
    error_init: t('CreatePostPage:error_init'),
    error_save: t('CreatePostPage:error_save'),
    error_upload: t('CreatePostPage:error_upload'),
    error_publish: t('CreatePostPage:error_publish'),
    error_empty_fields: t('CreatePostPage:error_empty_fields'),
  };
}
