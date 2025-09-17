import { Input } from '@/components/ui/input';

export default function LabelInput({
  id,
  label,
  onChange,
  errorMessage,
  children,
  ...props
}: {
  id: string;
  label?: string;
  errorMessage?: string;
  onChange: (value: string) => void;
  children?: React.ReactNode;
} & Omit<React.ComponentProps<'input'>, 'onChange'>) {
  return (
    <div className="flex flex-col gap-1.25 w-full">
      {label && <label id={id}>{label}</label>}
      <div className="flex flex-row gap-2.5">
        <Input id={id} onChange={(e) => onChange(e.target.value)} {...props} />
        {children}
      </div>
      {errorMessage && (
        <span className="text-sm text-red-500">{errorMessage}</span>
      )}
    </div>
  );
}
