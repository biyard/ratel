'use client';

interface SelectProps {
  value: number | string;
  onChange: (val: number | string) => void;
  options: (number | string)[];
  className?: string;
}

/**
 * @deprecated Use Select from '@/components/ui/select' instead.
 */
export default function Select({
  value,
  onChange,
  options,
  className = '',
}: SelectProps) {
  return (
    <select
      className={`cursor-pointer appearance-none bg-input-box-bg border border-neutral-600 light:border-input-box-border rounded-md px-3 py-2 text-text-primary text-sm w-full ${className}`}
      value={value}
      onChange={(e) => {
        const val = e.target.value;
        const parsed = isNaN(Number(val)) ? val : Number(val);
        onChange(parsed);
      }}
    >
      {options.map((opt) => (
        <option key={`opt-${opt}`} value={opt}>
          {opt}
        </option>
      ))}
    </select>
  );
}
