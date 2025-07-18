'use client';

import { Question } from '@/lib/api/models/survey';
import React, { useState } from 'react';
import SurveyQuestionEditor from './question/survey_question_editor';
import { AnswerType } from './question/answer_type_select';
import { v4 as uuidv4 } from 'uuid';
import SurveyViewer from './question/survey_viewer';
import { Add } from './add';
import { SurveyAnswer } from '../types';
import { Answer } from '@/lib/api/models/response';
import { SpaceStatus } from '@/lib/api/models/spaces';

export interface SpaceSurveyProps {
  isEdit?: boolean;
  status: SpaceStatus;
  questions: Question[];
  startDate: number;
  endDate: number;
  answer: SurveyAnswer;

  setAnswers: (answers: Answer[]) => void;
  setStartDate: (startDate: number) => void;
  setEndDate: (endDate: number) => void;
  onadd: (question: Question) => void;
  onupdate: (
    index: number,
    updated: {
      answerType: AnswerType;
      image_url?: string;
      title: string;
      options?: string[];
    },
  ) => void;
  onremove: (index: number) => void;
  onsend: () => void;
}

export default function SpaceSurvey({
  isEdit = false,
  status,
  questions,
  startDate,
  endDate,
  answer,

  setAnswers,
  setStartDate,
  setEndDate,
  onadd,
  onupdate,
  onremove,
  onsend,
}: SpaceSurveyProps) {
  return (
    <div className="flex flex-col w-full">
      {isEdit && status == SpaceStatus.Draft ? (
        <EditableSurvey
          questions={questions}
          startDate={startDate}
          endDate={endDate}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
          onadd={() => {
            onadd({
              answer_type: 'short_answer',
              title: '',
              description: '',
            });
          }}
          onupdate={onupdate}
          onremove={onremove}
        />
      ) : (
        <ViewSurvey
          isEdit={isEdit}
          status={status}
          answer={answer}
          setAnswers={setAnswers}
          questions={questions}
          startDate={startDate}
          endDate={endDate}
          onSend={onsend}
        />
      )}
    </div>
  );
}

function ViewSurvey({
  isEdit,
  status,
  answer,
  setAnswers,
  questions,
  startDate,
  endDate,
  onSend,
}: {
  isEdit: boolean;
  status: SpaceStatus;
  answer: SurveyAnswer;
  setAnswers: (answer: Answer[]) => void;
  questions: Question[];
  startDate: number;
  endDate: number;
  onSend: () => void;
}) {
  return (
    <div className="flex flex-col w-full gap-[10px]">
      <SurveyViewer
        isEdit={isEdit}
        status={status}
        startDate={startDate}
        endDate={endDate}
        questions={questions}
        answer={answer}
        setAnswers={setAnswers}
        onConfirm={onSend}
      />
    </div>
  );
}

function EditableSurvey({
  questions,
  onadd,
  onupdate,
  onremove,
}: {
  questions: Question[];

  startDate: number;
  endDate: number;

  setStartDate: (startDate: number) => void;
  setEndDate: (endDate: number) => void;
  onadd: () => void;
  onupdate: (
    index: number,
    updated: {
      answerType: AnswerType;
      title: string;
      image_url?: string;
      options?: string[];
    },
  ) => void;
  onremove: (index: number) => void;
}) {
  const [stableKeys, setStableKeys] = useState<string[]>(() =>
    questions.map(() => uuidv4()),
  );
  const handleAdd = () => {
    onadd();
    setStableKeys((prev) => [...prev, uuidv4()]);
  };

  const handleRemove = (index: number) => {
    onremove(index);
    setStableKeys((prev) => prev.filter((_, i) => i !== index));
  };

  return (
    <div className="flex flex-col w-full gap-2.5">
      {questions.map((question, index) => {
        return (
          <div key={stableKeys[index]}>
            <SurveyQuestionEditor
              index={index}
              answerType={question.answer_type}
              title={question.title}
              imageUrl={'image_url' in question ? question.image_url : ''}
              options={'options' in question ? question.options : []}
              onupdate={(updated) => {
                onupdate(index, {
                  answerType: updated.answerType,
                  title: updated.title,
                  image_url: updated.image_url,
                  options: updated.options ?? [],
                });
              }}
              onremove={(index: number) => {
                handleRemove(index);
              }}
            />
          </div>
        );
      })}
      <div className="relative flex items-center justify-center w-full py-6">
        <div
          className="absolute top-1/2 w-full h-0.25"
          style={{
            borderTop: '1px dashed transparent',
            borderImage:
              'repeating-linear-gradient(to right, #525252 0 8px, transparent 8px 16px) 1',
          }}
        />

        <div
          className="cursor-pointer z-10 bg-background flex items-center justify-center w-fit h-fit p-[13px] border border-neutral-500 rounded-full"
          onClick={handleAdd}
        >
          <Add className="w-4 h-4 stroke-neutral-500 text-neutral-500" />
        </div>
      </div>
    </div>
  );
}

// function ViewSurvey({}: { questions: Question[] }) {
//   return <div>view surveys</div>;
// }
