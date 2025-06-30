interface CustomCheckboxProps {
  isRounded?: boolean;
  checked: boolean;
  onChange: () => void;
  disabled?: boolean;
}

export default function CustomCheckbox({
  isRounded = false,
  checked,
  onChange,
  disabled = false,
}: CustomCheckboxProps) {
  return (
    <button
      type="button"
      onClick={onChange}
      aria-pressed={checked}
      className={`w-6 h-6 flex items-center justify-center border
        ${disabled ? 'bg-neutral-500 border-transparent' : checked ? 'bg-yellow-500 border-transparent' : 'bg-transparent border-neutral-500'}
        ${isRounded ? 'rounded-full' : 'rounded-sm'}
        transition-colors duration-150 `}
    >
      {checked && (
        <svg
          className="w-4 h-4 text-black"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={3}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M5 13l4 4L19 7"
          />
        </svg>
      )}
    </button>
  );
}
