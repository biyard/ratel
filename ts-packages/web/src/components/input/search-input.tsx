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
    if (e.key === 'Enter') {
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
      className="bg-transparent text-white placeholder:text-neutral-500 border border-[#464646] text-base outline-none w-full focus:border focus:border-primary p-[10px] rounded-sm"
    />
  );
}
