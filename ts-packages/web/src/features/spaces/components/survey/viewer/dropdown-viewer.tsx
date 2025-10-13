import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import Wrapper, { type WrapperProps } from './wrapper';
import { DropdownQuestion } from '@/types/survey-type';

interface DropdownViewerProps extends DropdownQuestion, WrapperProps {
  selectedOption: number | null;
  disabled?: boolean;
  onSelect: (optIndex: number) => void;
}
export default function DropdownViewer(props: DropdownViewerProps) {
  const { t, options, selectedOption, onSelect, disabled = false } = props;
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Wrapper {...props} />
      <Select
        disabled={disabled}
        value={selectedOption !== null ? options[selectedOption] : undefined}
        onValueChange={(value) => {
          const index = options.indexOf(value);
          if (index !== -1) {
            onSelect(index);
          }
        }}
      >
        <SelectTrigger className="w-full max-w-70">
          <SelectValue placeholder={t('choose')} />
        </SelectTrigger>
        <SelectContent>
          {options.map((option, optIndex) => (
            <SelectItem key={`dropdown-${optIndex}`} value={option}>
              {option}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
