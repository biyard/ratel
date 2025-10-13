import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { SurveyAnswerType } from '@/types/survey-type';
import { TFunction } from 'i18next';

export default function TypeSelect({
  t,
  value,
  onChange,
}: {
  t: TFunction<'Survey', undefined>;
  value: SurveyAnswerType;
  onChange: (val: SurveyAnswerType) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="border-input-box-border bg-card-bg focus:border-primary px-5 py-[10.5px] w-[260px] max-mobile:!w-full font-medium text-[15px]/[22.5px] text-neutral-600 rounded-lg focus:ring-primary !h-full">
        <SelectValue placeholder="Select an answer type" />
      </SelectTrigger>
      <SelectContent>
        {Object.values(SurveyAnswerType).map((type) => (
          <SelectItem key={type} value={type} className="text-neutral-600">
            {t(type)}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
