import { useState } from 'react';
import { useSpaceLayoutI18n } from '../space-layout-i18n';

export default function SpaceEndModal({
  onEnded,
  onClose,
}: {
  onEnded: () => void | Promise<void>;
  onClose?: () => void;
}) {
  const i18n = useSpaceLayoutI18n();
  const [isEnd, setIsEnd] = useState(false);

  const handleEnd = async () => {
    if (isEnd) return;

    setIsEnd(true);
    try {
      await onEnded();
    } finally {
      setIsEnd(false);
    }
  };

  return (
    <div className="flex flex-col mt-6 w-mobile max-tablet:w-full">
      <div className="mb-6 text-base font-medium text-center text-desc-text">
        {i18n.end_modal_desc}
      </div>

      <div className="flex flex-row gap-4 justify-end mt-4">
        <button
          onClick={onClose}
          className="px-10 text-base font-bold transition-colors hover:text-white min-w-30 py-[14.5px] bg-cancel-button-bg text-cancel-button-text light:hover:text-hover"
        >
          {i18n.cancel}
        </button>
        <button
          data-testid="start-space-button"
          onClick={handleEnd}
          disabled={isEnd}
          className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
            !isEnd
              ? 'bg-primary text-black hover:bg-primary/80'
              : 'bg-neutral-700 light:bg-neutral-300 text-neutral-500 cursor-not-allowed'
          } transition-colors`}
        >
          {isEnd ? i18n.end_modal_button_ending : i18n.end_modal_button_end}
        </button>
      </div>
    </div>
  );
}
