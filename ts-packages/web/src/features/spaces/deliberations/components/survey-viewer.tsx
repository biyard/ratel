import { Answer } from '@/lib/api/models/response';
import { usePopup } from '@/lib/contexts/popup-service';

import { logger } from '@/lib/logger';

import { useTranslation } from 'react-i18next';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';
import { SurveyAnswer } from '@/app/spaces/[id]/type';
import ObjectiveViewer from '@/app/spaces/[id]/_components/question/_component/viewer/objective-viewer';
import LinearScaleViewer from '@/app/spaces/[id]/_components/question/_component/viewer/linear-scale-viewer';
import DropdownViewer from '@/app/spaces/[id]/_components/question/_component/viewer/dropdown-viewer';
import SubjectiveViewer from '@/app/spaces/[id]/_components/question/_component/viewer/subjective-viewer';
import CheckPopup from '@/app/spaces/[id]/_components/question/check-popup';
import { useUserInfo } from '@/hooks/use-user-info';
import { SpacePublishState } from '@/features/deliberation-space/utils/deliberation.spaces.v3';
import { Poll } from '@/app/spaces/deliberation/[id]/types';

interface Question {
  title: string;
  answer_type: Answer['answer_type'];
  min_value?: number;
  max_value?: number;
  min_label?: string;
  max_label?: string;
  image_url?: string;
  is_required?: boolean;
  is_multi?: boolean;
  options?: string[];
}

export default function SurveyViewer({
  isEdit,
  survey,
  answer,
  publish,
  startDate,
  endDate,
  handleSetAnswers,
  handleSend,
}: {
  isEdit: boolean;
  startDate: number;
  endDate: number;
  survey: Poll;
  answer: SurveyAnswer;
  publish: SpacePublishState;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSend: () => Promise<void>;
}) {
  const { t } = useTranslation('PollSpace');
  const { data: userInfo } = useUserInfo();
  const userPk = userInfo?.pk || '';

  // const members = space.discussions.flatMap((discussion) => discussion.members);
  // const isMember = members.some((member) => member.id === userId);

  const questions: Question[] =
    survey.surveys.length != 0 ? survey.surveys[0].questions : [];

  const popup = usePopup();
  const is_completed = answer.is_completed;
  const answers: Answer[] = answer.answers;

  const now = Math.floor(Date.now() / 1000);

  const isLive = now >= startDate && now <= endDate;

  logger.debug('is completed:', is_completed, ' status:', status);

  const handleSelect = (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => {
    if (is_completed) return;

    const updated = [...answers];

    if (type === 'single_choice' || type === 'linear_scale') {
      updated[qIdx] = {
        answer_type: type,
        answer: optionIdx,
      } satisfies Answer;
    } else if (type === 'checkbox') {
      if (questions[qIdx].is_multi) {
        const existing =
          answers[qIdx]?.answer_type === 'checkbox'
            ? [
                ...((
                  answers[qIdx] as Extract<Answer, { answer_type: 'checkbox' }>
                ).answer ?? []),
              ]
            : [];
        const exists = existing.includes(optionIdx);
        const newAnswer = exists
          ? existing.filter((v) => v !== optionIdx)
          : [...existing, optionIdx];

        updated[qIdx] = {
          answer_type: 'checkbox',
          answer: newAnswer,
        } satisfies Answer;
      } else {
        updated[qIdx] = {
          answer_type: 'checkbox',
          answer: [optionIdx],
        } satisfies Answer;
      }
    } else if (type === 'multiple_choice') {
      const existing =
        answers[qIdx]?.answer_type === 'multiple_choice'
          ? [
              ...((
                answers[qIdx] as Extract<
                  Answer,
                  { answer_type: 'multiple_choice' }
                >
              ).answer ?? []),
            ]
          : [];
      const exists = existing.includes(optionIdx);
      const newAnswer = exists
        ? existing.filter((v) => v !== optionIdx)
        : [...existing, optionIdx];

      updated[qIdx] = {
        answer_type: 'multiple_choice',
        answer: newAnswer,
      } satisfies Answer;
    } else if (type === 'dropdown') {
      updated[qIdx] = {
        answer_type: 'dropdown',
        answer: optionIdx,
      } satisfies Answer;
    }

    handleSetAnswers(updated);
  };

  const handleInput = (
    qIdx: number,
    value: string,
    type: 'short_answer' | 'subjective',
  ) => {
    if (is_completed) return;

    const updated = [...answers];
    updated[qIdx] = {
      answer_type: type,
      answer: value,
    } satisfies Answer;

    handleSetAnswers(updated);
  };

  return (
    <div className="flex flex-col gap-2.5 w-full">
      {questions.map((q, index) => {
        const selected = answers[index];

        let selectedIndexes =
          q.answer_type === 'checkbox' && selected?.answer_type === 'checkbox'
            ? selected.answer
            : [];

        if (selectedIndexes && selectedIndexes.length === 0) {
          selectedIndexes =
            q.answer_type === 'multiple_choice' &&
            selected?.answer_type === 'multiple_choice'
              ? selected.answer
              : [];
        }

        return (
          <BorderSpaceCard key={index}>
            <div className="flex flex-col w-full gap-2.5">
              {(q.answer_type === 'single_choice' ||
                q.answer_type === 'multiple_choice' ||
                q.answer_type == 'checkbox') && (
                <ObjectiveViewer
                  answerType={q.answer_type}
                  isRequired={q.is_required ?? false}
                  isMulti={q.is_multi}
                  title={q.title}
                  imageUrl={q.image_url}
                  options={q.options}
                  selected={selected}
                  selectedIndexes={selectedIndexes ?? []}
                  index={index}
                  isCompleted={is_completed}
                  handleSelect={handleSelect}
                />
              )}

              {q.answer_type === 'linear_scale' && (
                <LinearScaleViewer
                  answerType={q.answer_type}
                  isRequired={q.is_required ?? false}
                  title={q.title}
                  minLabel={q.min_label}
                  minValue={q.min_value}
                  maxLabel={q.max_label}
                  maxValue={q.max_value}
                  selected={selected}
                  isCompleted={is_completed}
                  index={index}
                  handleSelect={handleSelect}
                />
              )}

              {q.answer_type === 'dropdown' && (
                <DropdownViewer
                  title={q.title}
                  isRequired={q.is_required ?? false}
                  isCompleted={is_completed}
                  selected={selected}
                  index={index}
                  options={q.options ?? []}
                  handleSelect={handleSelect}
                />
              )}

              {(q.answer_type === 'short_answer' ||
                q.answer_type === 'subjective') && (
                <SubjectiveViewer
                  answerType={q.answer_type}
                  isRequired={q.is_required ?? false}
                  title={q.title}
                  selected={selected}
                  index={index}
                  isCompleted={is_completed}
                  handleInput={handleInput}
                />
              )}
            </div>
          </BorderSpaceCard>
        );
      })}

      <div
        className={`flex flex-row w-full justify-end ${
          is_completed ||
          !isLive ||
          userPk === '' ||
          publish === SpacePublishState.Draft.toUpperCase() ||
          isEdit ||
          questions.length == 0
            ? 'hidden'
            : ''
        }`}
      >
        <div
          className="cursor-pointer flex flex-row w-[180px] h-fit py-[14px] px-[40px] justify-center items-center bg-primary hover:opacity-70 rounded-lg font-bold text-[15px] text-[#000203]"
          onClick={() => {
            popup
              .open(
                <CheckPopup
                  onContinue={() => {
                    handleSend();
                    popup.close();
                  }}
                  onClose={() => {
                    popup.close();
                  }}
                />,
              )
              .withTitle(t('check_title'));
          }}
        >
          {t('save')}
        </div>
      </div>
    </div>
  );
}
