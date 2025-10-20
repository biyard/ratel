'use client';

import CustomCheckbox from '@/components/checkbox/custom-checkbox';
// import { Answer } from '@/lib/api/models/response';
// import { Question } from '@/lib/api/models/survey';
import Title, { type TitleProps } from './title';
import { ChoiceQuestion } from '@/features/spaces/polls/types/poll-question';

interface ObjectiveViewerProps extends ChoiceQuestion, TitleProps {
  disabled?: boolean;
  selectedIndexes: number[];
  onSelect: (index: number) => void;
}
export default function ObjectiveViewer(props: ObjectiveViewerProps) {
  const {
    image_url,
    title,
    options,
    selectedIndexes,
    answer_type,
    disabled,
    onSelect,
  } = props;
  return (
    <>
      <Title {...props} />
      {image_url ? (
        <img
          className="object-contain rounded-lg max-h-70 w-fit"
          src={image_url}
          alt={title}
        />
      ) : (
        <></>
      )}
      <div className="flex flex-col gap-2">
        {options.map((option, optionIdx) => {
          return (
            <div
              key={`${answer_type}-${optionIdx}`}
              className="flex flex-row gap-3 justify-start items-center w-full h-fit"
            >
              <div className="w-4.5 h-4.5">
                <CustomCheckbox
                  checked={selectedIndexes.includes(optionIdx)}
                  onChange={() => onSelect(optionIdx)}
                  disabled={disabled}
                />
              </div>
              <div className="font-normal text-neutral-300 light:text-text-primary text-[15px]/[22.5px]">
                {option}
              </div>
            </div>
          );
        })}
      </div>
    </>
  );
}
