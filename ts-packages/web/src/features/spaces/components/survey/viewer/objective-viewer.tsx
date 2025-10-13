'use client';

import CustomCheckbox from '@/components/checkbox/custom-checkbox';
// import { Answer } from '@/lib/api/models/response';
// import { Question } from '@/lib/api/models/survey';
import Wrapper, { type WrapperProps } from './wrapper';
import { ChoiceQuestion } from '@/types/survey-type';

interface ObjectiveViewerProps extends ChoiceQuestion, WrapperProps {
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
      <Wrapper {...props} />
      {image_url ? (
        <img
          className="object-contain max-h-70 w-fit rounded-lg"
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
              className="flex flex-row w-full h-fit justify-start items-center gap-3"
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
