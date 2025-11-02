'use client';

export default function RadioButton({
  onClick,
  selected,
  disabled = false,
}: {
  onClick: () => void;
  selected: boolean;
  disabled?: boolean;
}) {
  return (
    <div className="flex items-center">
      <button
        onClick={disabled ? undefined : onClick}
        disabled={disabled}
        className={`w-6 h-6 rounded-full flex items-center justify-center transition-colors ${
          disabled && selected
            ? 'bg-[#fcb300] opacity-50 cursor-not-allowed'
            : disabled
              ? 'opacity-50 cursor-not-allowed border-2 border-[#6b6b6b]'
              : selected
                ? 'bg-[#fcb300] hover:bg-[#fcb300]/90'
                : 'border-2 border-[#6b6b6b] hover:border-[#6b6b6b]/80'
        }`}
      >
        {selected && (
          <svg
            className="w-3 h-3 text-black"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clipRule="evenodd"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
