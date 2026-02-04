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
