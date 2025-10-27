import { useTranslation } from 'react-i18next';
import { useState } from 'react';

export default function SpaceDeleteModal({
  spaceName,
  onDelete,
  onClose,
}: {
  spaceName: string;
  onDelete: () => void | Promise<void>;
  onClose?: () => void;
}) {
  const { t } = useTranslation('Space');
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
    <div className="w-[500px] max-tablet:w-full flex flex-col mt-6">
      <div className="text-center font-medium text-desc-text text-base mb-6">
        {t('delete_warning')}
      </div>

      <div className="mb-6">
        <label
          htmlFor="spaceNameVerification"
          className="block text-desc-text text-sm mb-2"
        >
          {t('delete_label')}
        </label>
        <input
          id="spaceNameVerification"
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          className="w-full p-3 bg-input-box-bg border border-input-box-border rounded-lg text-text-primary focus:outline-none focus:ring-2 focus:ring-primary"
          placeholder={t('delete_placeholder', { spaceName })}
        />
      </div>

      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onClose}
          className="min-w-30 px-10 py-[14.5px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-white light:hover:text-hover transition-colors"
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
