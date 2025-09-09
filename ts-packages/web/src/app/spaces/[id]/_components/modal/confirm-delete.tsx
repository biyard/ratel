'use client';

import { useTranslations } from 'next-intl';
import React, { useState } from 'react';

export default function DeleteSpacePopup({
  spaceName,
  onDelete,
  onClose,
}: {
  spaceName: string;
  onDelete: () => void | Promise<void>;
  onClose?: () => void;
}) {
  const t = useTranslations('Space');
  const [inputValue, setInputValue] = useState('');
  const [isConfirmed, setIsConfirmed] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setInputValue(value);
    setIsConfirmed(value === spaceName);
  };
  const handleDelete = async () => {
    if (!isConfirmed || isDeleting) return;

    setIsDeleting(true);
    try {
      await onDelete();
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div className="w-[500px] flex flex-col mt-6">
      <div className="text-center font-bold text-create-space-label text-[24px] mb-6">
        {t.rich('delete_title', {
          name: () => <span>'{spaceName}'</span>,
        })}
      </div>

      <div className="text-center font-medium text-create-space-desc text-base mb-6">
        {t('delete_warning')}
      </div>

      <div className="mb-6">
        <label
          htmlFor="spaceNameVerification"
          className="block text-create-space-desc text-sm mb-2"
        >
          {t('delete_label')}
        </label>
        <input
          id="spaceNameVerification"
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          className="w-full p-3 bg-input-box-bg border border-input-box-border light:border-foreground rounded-lg text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
          placeholder={t('delete_placeholder', { spaceName })}
        />
      </div>

      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onClose}
          className="min-w-30 px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400 hover:text-white light:hover:text-hover transition-colors"
        >
          {t('cancel')}
        </button>
        <button
          onClick={handleDelete}
          disabled={!isConfirmed || isDeleting}
          className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
            isConfirmed && !isDeleting
              ? 'bg-red-600 text-white hover:bg-red-700'
              : 'bg-neutral-700 light:bg-neutral-300 text-neutral-500 cursor-not-allowed'
          } transition-colors`}
        >
          {isDeleting ? t('deleting') : t('delete_button')}
        </button>
      </div>
    </div>
  );
}
