'use client';
import React, { useState } from 'react';
import AnswerTypeSelect, { AnswerType } from './answer-type-select';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { DialPad, Image2 } from '@/components/icons';
import FileUploader from '@/components/file-uploader';
import Image from 'next/image';
import LinearScaleSelection from './_component/linear-scale-selection';
import ObjectiveOption from './_component/objective-option';
import LabelSwitchButton from './_component/label-switch-button';
import { useTranslations } from 'next-intl';

export default function SurveyQuestionEditor({
  index,
  answerType,
  imageUrl,
  title,
  options,
  isMulti,
  isRequired,
  min,
  max,
  minLabel,
  maxLabel,
  onupdate,
  onremove,
}: {
  index: number;
  answerType: AnswerType;
  title: string;
  imageUrl?: string;
  options?: string[];
  isMulti?: boolean;
  isRequired?: boolean;
  min?: number;
  max?: number;
  minLabel?: string;
  maxLabel?: string;
  onupdate?: (updated: {
    answerType: AnswerType;
    title: string;
    image_url?: string;
    options?: string[];
    min_label?: string;
    max_label?: string;
    min_value?: number;
    max_value?: number;
    is_multi?: boolean;
    is_required?: boolean;
  }) => void;
  onremove?: (index: number) => void;
}) {
  const t = useTranslations('PollSpace');
  const [questionType, setQuestionType] = useState<AnswerType>(answerType);
  const [questionTitle, setQuestionTitle] = useState(title);
  const [questionOptions, setQuestionOptions] = useState<string[]>(
    options || [''],
  );
  const [questionImage, setQuestionImage] = useState(imageUrl);
  const [questionMulti, setQuestionMulti] = useState(isMulti);
  const [questionRequired, setQuestionRequired] = useState(isRequired);
  const [minValue] = useState<number>(min ?? 1);
  const [maxValue, setMaxValue] = useState<number>(max ?? 10);

  const [labels, setLabels] = useState<Record<number, string>>(() => ({
    ...(min !== undefined && minLabel !== undefined ? { [min]: minLabel } : {}),
    ...(max !== undefined && maxLabel !== undefined ? { [max]: maxLabel } : {}),
  }));

  const updateQuestion = (
    overrides: Partial<Parameters<NonNullable<typeof onupdate>>[0]>,
  ) => {
    if (!onupdate) return;
    onupdate({
      answerType: questionType,
      title: questionTitle,
      image_url: questionImage,
      is_multi: questionMulti,
      is_required: questionRequired,
      options:
        questionType.includes('choice') ||
        questionType.includes('checkbox') ||
        questionType.includes('dropdown')
          ? questionOptions
          : undefined,
      min_value: minValue,
      max_value: maxValue,
      min_label: labels[minValue],
      max_label: labels[maxValue],
      ...overrides,
    });
  };

  const handleMaxValueChange = (val: number) => {
    setMaxValue(val);
    updateQuestion({
      max_value: val,
      max_label: labels[val],
    });
  };

  const handleLabelChange = (targetValue: number, label: string) => {
    const updatedLabels = {
      ...labels,
      [targetValue]: label,
    };
    setLabels(updatedLabels);
    updateQuestion({
      min_label: updatedLabels[minValue],
      max_label: updatedLabels[maxValue],
    });
  };

  const handleOptionChange = (idx: number, value: string) => {
    const newOptions = [...questionOptions];
    newOptions[idx] = value;
    setQuestionOptions(newOptions);
    updateQuestion({ options: newOptions });
  };

  const handleRequiredChange = (value: boolean) => {
    setQuestionRequired(value);
    updateQuestion({ is_required: value });
  };

  const handleMultiChange = (value: boolean) => {
    setQuestionMulti(value);
    updateQuestion({ is_multi: value });
  };

  const handleImageChange = (value: string) => {
    setQuestionImage(value);
    updateQuestion({ image_url: value });
  };

  const handleTitleChange = (value: string) => {
    setQuestionTitle(value);
    updateQuestion({ title: value });
  };

  const handleTypeChange = (val: AnswerType) => {
    setQuestionType(val);
    updateQuestion({ answerType: val });
  };

  const addOption = () => {
    const newOptions = [...questionOptions, ''];
    setQuestionOptions(newOptions);
    updateQuestion({ options: newOptions });
  };

  const handleRemoveOption = (optIdx: number) => {
    const newOptions = questionOptions.filter((_, idx) => idx !== optIdx);
    setQuestionOptions(newOptions);
    updateQuestion({ options: newOptions });
  };

  return (
    <div className="flex flex-col w-full bg-component-bg rounded-[10px] px-4 pb-5 pt-1">
      <div className="flex flex-row w-full justify-center items-center mb-2.5">
        <DialPad className="w-6 h-6" />
      </div>
      <div className="flex flex-col w-full gap-2.5">
        <div className="flex gap-2 max-tablet:flex-col">
          <AnswerTypeSelect value={questionType} onChange={handleTypeChange} />
          <Input
            className="bg-input-box-bg border border-input-box-border rounded-lg w-full px-4 !py-5.5 font-medium text-[15px]/[22.5px] text-foreground placeholder:text-neutral-600 "
            type="text"
            placeholder={t('title_hint')}
            value={questionTitle}
            onChange={(e) => handleTitleChange(e.target.value)}
          />
          {questionType == 'checkbox' ||
          questionType === 'dropdown' ||
          questionType === 'single_choice' ||
          questionType === 'multiple_choice' ||
          questionType === 'linear_scale' ? (
            <FileUploader onUploadSuccess={handleImageChange}>
              <div className="cursor-pointer flex flex-row w-fit h-fit p-[10.59px] bg-white border border-transparent light:border-[#e5e5e5] rounded-lg">
                <Image2 className="w-[22.81px] h-[22.81px] " />
              </div>
            </FileUploader>
          ) : (
            <></>
          )}
        </div>
        {imageUrl ? (
          <Image
            width={300}
            height={300}
            className="object-contain max-w-75"
            src={imageUrl}
            alt={title || 'Question Title'}
          />
        ) : (
          <></>
        )}
        <div className="flex flex-col mt-2.5 gap-2.5">
          {(questionType === 'checkbox' ||
            questionType === 'dropdown' ||
            questionType === 'single_choice' ||
            questionType === 'multiple_choice') && (
            <ObjectiveOption
              questionOptions={questionOptions}
              index={index}
              questionType={questionType}
              handleOptionChange={handleOptionChange}
              handleRemoveOption={handleRemoveOption}
              addOption={addOption}
            />
          )}
          {questionType === 'linear_scale' && (
            <LinearScaleSelection
              minValue={minValue}
              maxValue={maxValue}
              setMaxValue={handleMaxValueChange}
              labels={labels}
              setLabels={handleLabelChange}
            />
          )}
        </div>
        <div className="flex flex-row w-full justify-end items-center">
          <div className="flex flex-row w-fit gap-10">
            {questionType == 'checkbox' && (
              <LabelSwitchButton
                bgColor="bg-blue-500"
                textColor="text-blue-500"
                label={t('multiple_selection')}
                value={isMulti ?? false}
                onChange={handleMultiChange}
              />
            )}
            <LabelSwitchButton
              bgColor="bg-red-500"
              textColor="text-red-500"
              label={t('required')}
              value={isRequired ?? false}
              onChange={handleRequiredChange}
            />
            <div
              className="cursor-pointer flex flex-row w-fit gap-1.25 items-center"
              onClick={() => onremove?.(index)}
            >
              <div className="text-[15px] text-neutral-500 font-medium cursor-pointer">
                {t('delete')}
              </div>
              <Trash2 className="w-4.5 h-4.5 stroke-neutral-500 cursor-pointer" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
