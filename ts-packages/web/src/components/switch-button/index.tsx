'use client';

export default function SwitchButton({
  value,
  onChange,
  color,
}: {
  value: boolean;
  onChange: (val: boolean) => void;
  color?: string;
}) {
  return (
    <div
      onClick={() => onChange(!value)}
      className={`cursor-pointer w-11 h-5 flex items-center rounded-full p-0.5 transition-colors duration-300 ${
        value ? color : 'bg-gray-400'
      }`}
    >
      <div
        className={`bg-white w-4 h-4 rounded-full shadow-md transform duration-300 ${
          value ? 'translate-x-6' : ''
        }`}
      />
    </div>
  );
}
