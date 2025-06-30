'use client';

import { Question } from '@/lib/api/models/survey';
import React, { useState } from 'react';
import SurveyQuestionEditor from './question/survey_question_editor';
import { AnswerType } from './question/answer_type_select';
import { v4 as uuidv4 } from 'uuid';
import SurveyViewer from './question/survey_viewer';
import { format } from 'date-fns';
import { Add } from './add';
import CustomCalendar from '@/components/calendar-picker/calendar-picker';
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
    updated: { answerType: AnswerType; title: string; options?: string[] },
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
      {isEdit ? (
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
  status,
  answer,
  setAnswers,
  questions,
  startDate,
  endDate,
  onSend,
}: {
  status: SpaceStatus;
  answer: SurveyAnswer;
  setAnswers: (answer: Answer[]) => void;
  questions: Question[];
  startDate: number;
  endDate: number;
  onSend: () => void;
}) {
  const formattedDate = `${format(new Date(startDate * 1000), 'dd MMM, yyyy')} - ${format(new Date(endDate * 1000), 'dd MMM, yyyy')}`;
  return (
    <div className="flex flex-col w-full gap-[10px]">
      <div className="flex flex-row w-full justify-between items-center">
        <div className="text-base text-white font-semibold">Period</div>
        <div className="text-sm text-white font-normal">{formattedDate}</div>
      </div>
      <SurveyViewer
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
  startDate,
  endDate,

  setStartDate,
  setEndDate,
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
    updated: { answerType: AnswerType; title: string; options?: string[] },
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
      <div className="flex flex-wrap w-full justify-between items-center gap-[10px] mb-[10px]">
        <div className="font-medium text-neutral-300 text-[15px] w-[80px]">
          Period
        </div>
        <div className="flex flex-row gap-[10px] items-center flex-wrap">
          <div className="flex flex-row gap-[10px]">
            <CustomCalendar
              value={startDate * 1000}
              onChange={(date) => {
                const newStart = Math.floor(date / 1000);
                setStartDate(newStart);
                // update(newStart, endTime, title, desc);
              }}
            />
          </div>
          <div className="w-[20px] h-[1px] bg-neutral-500" />
          <div className="flex flex-row gap-[10px]">
            <CustomCalendar
              value={endDate * 1000}
              onChange={(date) => {
                const newEnd = Math.floor(date / 1000);
                setEndDate(newEnd);
                // update(startTime, newEnd, title, desc);
              }}
            />
          </div>
        </div>
      </div>
      {questions.map((question, index) => {
        return (
          <div key={stableKeys[index]}>
            <SurveyQuestionEditor
              index={index}
              answerType={question.answer_type}
              title={question.title}
              options={'options' in question ? question.options : []}
              onupdate={(updated) => {
                onupdate(index, {
                  answerType: updated.answerType,
                  title: updated.title,
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
      <div
        onClick={() => {
          handleAdd();
        }}
        className="bg-transparent border-2 border-dashed border-neutral-700 rounded-md h-50 flex flex-col items-center justify-center cursor-pointer hover:bg-neutral-800 transition gap-[10px]"
      >
        <div className="flex flex-row w-[45px] h-[45px] justify-center items-center rounded-full border border-neutral-500">
          <Add className="w-5 h-5 stroke-neutral-500 text-neutral-500" />
        </div>
        <span className=" text-neutral-500 font-medium text-base">
          Add New Question
        </span>
      </div>
    </div>
  );
}

// function ViewSurvey({}: { questions: Question[] }) {
//   return <div>view surveys</div>;
// }
