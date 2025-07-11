'use client';
import React, { useState } from 'react';
import AnswerTypeSelect, { AnswerType } from './answer_type_select';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { DialPad, DialPad2, Remove, Image2 } from '@/components/icons';
import FileUploader from '@/components/file-uploader';
import Image from 'next/image';

export default function SurveyQuestionEditor({
  index,
  answerType,
  imageUrl,
  title,
  options,
  onupdate,
  onremove,
}: {
  index: number;
  answerType: AnswerType;
  title: string;
  imageUrl?: string;
  options?: string[];
  onupdate?: (updated: {
    answerType: AnswerType;
    title: string;
    image_url?: string;
    options?: string[];
  }) => void;
  onremove?: (index: number) => void;
}) {
  const [questionType, setQuestionType] = useState<AnswerType>(answerType);
  const [questionTitle, setQuestionTitle] = useState(title);
  const [questionOptions, setQuestionOptions] = useState<string[]>(
    options || [''],
  );
  const [questionImage, setQuestionImage] = useState(imageUrl);

  const handleOptionChange = (idx: number, value: string) => {
    const newOptions = [...questionOptions];
    newOptions[idx] = value;
    setQuestionOptions(newOptions);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: questionTitle,
        image_url: questionImage,
        options: questionType.includes('choice') ? newOptions : undefined,
      });
    }
  };

  const handleImageChange = (value: string) => {
    setQuestionImage(value);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: questionTitle,
        image_url: value,
        options: questionType.includes('choice') ? questionOptions : undefined,
      });
    }
  };

  const handleTitleChange = (value: string) => {
    setQuestionTitle(value);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: value,
        image_url: questionImage,
        options: questionType.includes('choice') ? questionOptions : undefined,
      });
    }
  };

  const handleTypeChange = (val: AnswerType) => {
    setQuestionType(val);
    if (onupdate) {
      onupdate({
        answerType: val,
        title: questionTitle,
        image_url: questionImage,
        options: val.includes('choice') ? questionOptions : undefined,
      });
    }
  };

  const addOption = () => {
    const newOptions = [...questionOptions, ''];
    setQuestionOptions(newOptions);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: questionTitle,
        image_url: questionImage,
        options: newOptions,
      });
    }
  };

  const handleRemoveOption = (optIdx: number) => {
    const newOptions = questionOptions.filter((_, idx) => idx !== optIdx);
    setQuestionOptions(newOptions);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: questionTitle,
        image_url: questionImage,
        options: questionType.includes('choice') ? newOptions : undefined,
      });
    }
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
            className="bg-neutral-800 border border-neutral-700 rounded-lg w-full px-4 !py-5.5 font-medium text-[15px]/[22.5px] text-white placeholder:text-neutral-600 "
            type="text"
            placeholder="Title"
            value={questionTitle}
            onChange={(e) => handleTitleChange(e.target.value)}
          />
          {questionType == 'single_choice' ||
          questionType == 'multiple_choice' ? (
            <FileUploader onUploadSuccess={handleImageChange}>
              <div className="cursor-pointer flex flex-row w-fit h-fit p-[10.59px] bg-white rounded-lg">
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
          {(questionType === 'single_choice' ||
            questionType === 'multiple_choice') && (
            <div className="flex flex-col gap-2">
              {questionOptions.map((opt, idx) => (
                <div
                  key={`option-${index}-${idx}`}
                  className="flex gap-2.5 items-center"
                >
                  <DialPad2 className="w-6 h-6" />
                  <Input
                    className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-normal text-base/[24px] placeholder:text-neutral-600 text-neutral-300 rounded-none"
                    type="text"
                    placeholder={`Type something...`}
                    value={opt}
                    onChange={(e) => handleOptionChange(idx, e.target.value)}
                  />
                  <Remove
                    className="cursor-pointer w-5 h-5 stroke-neutral-400 text-neutral-400"
                    onClick={() => handleRemoveOption(idx)}
                  />
                </div>
              ))}
              <button
                onClick={addOption}
                className="cursor-pointer text-sm text-neutral-500 font-semibold text-left mt-2"
              >
                + Add Option
              </button>
            </div>
          )}
        </div>

        <div className="flex flex-row w-full justify-end items-center">
          <div
            className="cursor-pointer flex flex-row w-fit gap-[5px] items-center"
            onClick={() => onremove?.(index)}
          >
            <div className="text-[15px] text-neutral-500 font-medium cursor-pointer">
              Delete
            </div>
            <Trash2 className="w-[18px] h-[18px] stroke-neutral-500 cursor-pointer" />
          </div>
        </div>
      </div>
    </div>
  );
}
