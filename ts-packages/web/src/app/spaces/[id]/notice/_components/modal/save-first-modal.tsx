'use client';

import { useTranslations } from 'next-intl';

export default function SaveThenPublishPopup({
  onJustPublish,
  onSaveAndPublish,
}: {
  onJustPublish: () => void;
  onSaveAndPublish: () => void;
}) {
  const t = useTranslations('NoticeSpace');

  return (
    <div className="w-[500px] flex flex-col">
      {/* Header */}
      <div className="text-center font-bold text-text-primary text-[24px] mb-6 mt-2">
        {t('save_first_title')}
      </div>

      {/* Body */}
      <div className="text-center font-medium text-desc-text text-base mb-8">
        {t('unsaved_desc_line1')}
        <br />
        {t('unsaved_desc_line2')}
        <br />
        {t('unsaved_desc_line3')}
        <br />
        <br />
        {t.rich('public_warning_rich', {
          b: (chunks) => <span className="font-bold">{chunks}</span>,
        })}
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        <button
          onClick={onJustPublish}
          className="flex-1 py-[14.5px] font-bold bg-cancel-button-bg text-cancel-button-text text-base rounded-[10px] hover:bg-hover transition-colors"
        >
          {t('just_publish')}
        </button>

        <button
          onClick={onSaveAndPublish}
          className="flex-1 py-[14.5px] bg-submit-button-bg font-bold text-submit-button-text text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          {t('save_and_publish')}
        </button>
      </div>
    </div>
  );
}
