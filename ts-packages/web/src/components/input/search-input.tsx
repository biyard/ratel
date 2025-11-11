export default function SearchInput({
  value,
  setValue,
  onenter,
  placeholder,
}: {
  value: string;
  placeholder: string;
  setValue: (value: string) => void;
  onenter: () => void;
}) {
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' || e.key === ',' || e.key === ' ') {
      onenter();
    }
  };

  return (
    <input
      type="text"
      placeholder={placeholder}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      onKeyDown={handleKeyDown}
      className="w-full text-base rounded-sm border outline-none focus:border text-text-primary placeholder:text-neutral-500 border-input-box-border bg-input-box-bg p-[10px] focus:border-primary"
    />
  );
}
