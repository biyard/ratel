import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import Title, { type TitleProps } from './title';
import { DropdownQuestion } from '@/features/spaces/polls/types/poll-question';

interface DropdownViewerProps extends DropdownQuestion, TitleProps {
  selectedOption: number | null;
  disabled?: boolean;
  onSelect: (optIndex: number) => void;
}
export default function DropdownViewer(props: DropdownViewerProps) {
  const { t, options, selectedOption, onSelect, disabled = false } = props;
  return (
    <div className="flex flex-col gap-2.5 w-full">
      <Title {...props} />
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
          <SelectValue placeholder={t('dropdown_select_placeholder')} />
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
