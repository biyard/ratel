'use client';
import React, { useState } from 'react';
import AnswerTypeSelect, { AnswerType } from './answer_type_select';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { EditOption } from '@/components/icons';

export default function SurveyQuestionEditor({
  index,
  answerType,
  title,
  options,
  onupdate,
  onremove,
}: {
  index: number;
  answerType: AnswerType;
  title: string;
  options?: string[];
  onupdate?: (updated: {
    answerType: AnswerType;
    title: string;
    options?: string[];
  }) => void;
  onremove?: (index: number) => void;
}) {
  const [questionType, setQuestionType] = useState<AnswerType>(answerType);
  const [questionTitle, setQuestionTitle] = useState(title);
  const [questionOptions, setQuestionOptions] = useState<string[]>(
    options || [''],
  );

  const handleOptionChange = (idx: number, value: string) => {
    const newOptions = [...questionOptions];
    newOptions[idx] = value;
    setQuestionOptions(newOptions);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: questionTitle,
        options: questionType.includes('choice') ? newOptions : undefined,
      });
    }
  };

  const handleTitleChange = (value: string) => {
    setQuestionTitle(value);
    if (onupdate) {
      onupdate({
        answerType: questionType,
        title: value,
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
        options: questionType.includes('choice') ? newOptions : undefined,
      });
    }
  };

  return (
    <div className="flex flex-col w-full border border-neutral-700 rounded-md p-4 gap-3">
      <div className="flex gap-2 max-tablet:flex-col">
        <AnswerTypeSelect value={questionType} onChange={handleTypeChange} />
        <Input
          type="text"
          placeholder="Input Question Title."
          value={questionTitle}
          onChange={(e) => handleTitleChange(e.target.value)}
        />
      </div>

      {(questionType === 'single_choice' ||
        questionType === 'multiple_choice') && (
        <div className="flex flex-col gap-2">
          {questionOptions.map((opt, idx) => (
            <div
              key={`option-${index}-${idx}`}
              className="flex gap-2 items-center"
            >
              <div className="w-4 h-4 rounded-full border border-neutral-400" />
              <Input
                type="text"
                placeholder={`Option ${idx + 1}`}
                value={opt}
                onChange={(e) => handleOptionChange(idx, e.target.value)}
              />
              <EditOption
                className="cursor-pointer w-4 h-4 stroke-neutral-400 text-neutral-400"
                onClick={() => handleRemoveOption(idx)}
              />
            </div>
          ))}
          <button
            onClick={addOption}
            className="cursor-pointer text-sm text-neutral-500 font-semibold mt-1 text-left"
          >
            + Add Option
          </button>
        </div>
      )}

      <div className="flex flex-row w-full justify-end items-center">
        <div
          className="cursor-pointer flex flex-row w-fit gap-[5px]"
          onClick={() => onremove?.(index)}
        >
          <div className="text-sm text-neutral-500 font-medium cursor-pointer">
            Delete
          </div>
          <Trash2 className="w-[18px] h-[18px] stroke-neutral-500 cursor-pointer" />
        </div>
      </div>
    </div>
  );
}
