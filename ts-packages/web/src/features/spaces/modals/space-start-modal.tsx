import { useTranslation } from 'react-i18next';
import { useState } from 'react';

export default function SpaceStartModal({
  onStarted,
  onClose,
}: {
  onStarted: () => void | Promise<void>;
  onClose?: () => void;
}) {
  const { t } = useTranslation('Space');
  const [isStarting, setIsStarting] = useState(false);

  const handleStart = async () => {
    if (isStarting) return;

    setIsStarting(true);
    try {
      await onStarted();
    } finally {
      setIsStarting(false);
    }
  };

  return (
    <div className="w-[500px] max-tablet:w-full flex flex-col mt-6">
      <div className="text-center font-medium text-desc-text text-base mb-6">
        {t('start_warning')}
      </div>

      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onClose}
          className="min-w-30 px-10 py-[14.5px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-white light:hover:text-hover transition-colors"
        >
          {t('cancel')}
        </button>
        <button
          onClick={handleStart}
          disabled={isStarting}
          className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
            !isStarting
              ? 'bg-primary text-black hover:bg-primary/80'
              : 'bg-neutral-700 light:bg-neutral-300 text-neutral-500 cursor-not-allowed'
          } transition-colors`}
        >
          {isStarting ? t('starting') : t('start_button')}
        </button>
      </div>
    </div>
  );
}
