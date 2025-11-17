import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import Title, { type TitleProps } from './title';
import { ChoiceQuestion } from '@/features/spaces/polls/types/poll-question';
import { Input } from '@/components/ui/input';

interface ObjectiveViewerProps extends ChoiceQuestion, TitleProps {
  disabled?: boolean;
  selectedIndexes: number[];
  allow_other?: boolean;
  onSelect: (index: number) => void;
  otherValue?: string;
  onChangeOther?: (value: string) => void;
}

const OTHER_LABEL = 'Others';

export default function ObjectiveViewer(props: ObjectiveViewerProps) {
  const {
    image_url,
    title,
    options,
    selectedIndexes,
    answer_type,
    disabled,
    allow_other = false,
    onSelect,
    otherValue,
    onChangeOther,
  } = props;

  const otherIndex =
    allow_other && options.includes(OTHER_LABEL)
      ? options.indexOf(OTHER_LABEL)
      : -1;

  return (
    <>
      <Title {...props} />
      {image_url ? (
        <img
          className="object-contain rounded-lg max-h-70 w-fit"
          src={image_url}
          alt={title}
        />
      ) : null}

      <div className="flex flex-col gap-2">
        {options.map((option, optionIdx) => {
          const isOther =
            allow_other && optionIdx === otherIndex && option === OTHER_LABEL;

          const checked = selectedIndexes.includes(optionIdx);

          return (
            <div
              key={`${answer_type}-${optionIdx}`}
              className="flex flex-row gap-3 justify-start items-center w-full h-fit"
            >
              <div className="w-4.5 h-4.5">
                <CustomCheckbox
                  checked={checked}
                  onChange={() => onSelect(optionIdx)}
                  disabled={disabled}
                />
              </div>

              {isOther ? (
                <Input
                  className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 light:text-text-primary rounded-none"
                  type="text"
                  placeholder={'Input the option.'}
                  value={otherValue ?? ''}
                  onChange={(e) => onChangeOther?.(e.target.value)}
                  disabled={disabled || !checked}
                />
              ) : (
                <div className="font-normal text-neutral-300 light:text-text-primary text-[15px]/[22.5px]">
                  {option}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </>
  );
}
