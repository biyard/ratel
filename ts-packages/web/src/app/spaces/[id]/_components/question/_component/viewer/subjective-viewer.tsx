'use client';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Answer } from '@/lib/api/models/response';
import Wrapper from './_components/wrapper';
import { useTranslation } from 'react-i18next';

export default function SubjectiveViewer({
  title,
  isRequired,
  answerType,
  selected,
  index,
  isCompleted,

  handleInput,
}: {
  title: string;
  isRequired: boolean;
  answerType: Answer['answer_type'];
  selected: Answer;
  index: number;
  isCompleted: boolean;

  handleInput: (
    qIdx: number,
    value: string,
    type: 'short_answer' | 'subjective',
  ) => void;
}) {
  const { t } = useTranslation('PollSpace');

  return (
    <div className="flex flex-col w-full gap-[10px]">
      <Wrapper
        isRequired={isRequired}
        answerType={selected?.answer_type}
        isMulti={false}
        title={title}
      />
      {answerType === 'short_answer' ? (
        <Input
          type="text"
          placeholder={t('subjective_hint')}
          className=" bg-input-box-bg border border-input-box-border text-base text-text-primary placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={
            selected?.answer_type === 'short_answer'
              ? (selected.answer ?? '')
              : ''
          }
          onChange={(e) => handleInput(index, e.target.value, 'short_answer')}
          disabled={isCompleted}
        />
      ) : (
        <Textarea
          rows={7}
          placeholder={t('subjective_hint')}
          className="bg-input-box-bg border border-input-box-border min-h-[185px]  text-base text-text-primary placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
          value={
            selected?.answer_type === 'subjective'
              ? (selected.answer ?? '')
              : ''
          }
          onChange={(e) => handleInput(index, e.target.value, 'subjective')}
          disabled={isCompleted}
        />
      )}
    </div>
  );
}
