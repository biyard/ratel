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
    <div className="flex flex-col mt-6 w-[500px] max-tablet:w-full">
      <div className="mb-6 text-base font-medium text-center text-desc-text">
        {t('start_warning')}
      </div>

      <div className="flex flex-row gap-4 justify-end mt-4">
        <button
          onClick={onClose}
          className="px-10 text-base font-bold transition-colors hover:text-white min-w-30 py-[14.5px] bg-cancel-button-bg text-cancel-button-text light:hover:text-hover"
        >
          {t('cancel')}
        </button>
        <button
          data-testid="start-space-button"
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
