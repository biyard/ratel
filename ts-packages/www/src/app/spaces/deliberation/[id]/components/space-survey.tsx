'use client';

import { Question, ShortAnswerQuestion } from '@/lib/api/models/survey';
import { useState } from 'react';
import { v4 as uuidv4 } from 'uuid';
import { SpaceStatus } from '@/lib/api/models/spaces';

import { Answer } from '@/lib/api/models/response';
import { Poll } from '../types';
import { SurveyAnswer } from '@/app/spaces/[id]/type';
import { AnswerType } from '@/app/spaces/[id]/_components/question/answer-type-select';
import SurveyQuestionEditor from '@/app/spaces/[id]/_components/question/survey-question-editor';
import { Add } from '@/components/icons';
import SurveyViewer from './survey-viewer';
import { DeliberationSpace } from '@/lib/api/ratel/spaces/deliberation-spaces.v3';

export default function SpaceSurvey({
  isEdit,
  startDate,
  endDate,
  survey,
  answer,
  status,
  space,
  handleSetAnswers,
  handleSend,
  handleUpdateSurvey,
}: {
  isEdit: boolean;
  startDate: number;
  endDate: number;
  survey: Poll;
  answer: SurveyAnswer;
  status: SpaceStatus;
  space: DeliberationSpace;
  handleUpdateSurvey: (survey: Poll) => void;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSend: () => Promise<void>;
}) {
  return (
    <div className="flex flex-col w-full">
      {isEdit && status == SpaceStatus.Draft ? (
        <EditableSurvey
          survey={survey}
          handleUpdateSurvey={handleUpdateSurvey}
        />
      ) : (
        <ViewSurvey
          isEdit={isEdit}
          startDate={startDate}
          endDate={endDate}
          survey={survey}
          answer={answer}
          status={status}
          handleSetAnswers={handleSetAnswers}
          handleSend={handleSend}
          space={space}
        />
      )}
    </div>
  );
}

function ViewSurvey({
  space,
  isEdit,
  startDate,
  endDate,
  survey,
  answer,
  handleSetAnswers,
  handleSend,
}: {
  isEdit: boolean;
  startDate: number;
  endDate: number;
  survey: Poll;
  answer: SurveyAnswer;
  status: SpaceStatus;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSend: () => Promise<void>;
  space: DeliberationSpace;
}) {
  return (
    <div className="flex flex-col w-full gap-[10px]">
      <SurveyViewer
        isEdit={isEdit}
        startDate={startDate}
        endDate={endDate}
        survey={survey}
        answer={answer}
        publish={space.publish_state}
        handleSetAnswers={handleSetAnswers}
        handleSend={handleSend}
      />
    </div>
  );
}

function EditableSurvey({
  survey,
  handleUpdateSurvey,
}: {
  survey: Poll;
  handleUpdateSurvey: (survey: Poll) => void;
}) {
  const questions =
    survey.surveys.length != 0 ? survey.surveys[0].questions : [];

  const [stableKeys, setStableKeys] = useState<string[]>(() =>
    questions.map(() => uuidv4()),
  );

  const handleAddQuestion = () => {
    const question: ShortAnswerQuestion = {
      answer_type: 'short_answer',
      title: '',
      description: '',
    };

    const existingSurvey = survey.surveys[0] ?? {
      started_at: 0,
      ended_at: 10000000000,
      questions: [],
    };

    const updatedSurvey = {
      ...existingSurvey,
      questions: [...existingSurvey.questions, question],
    };

    handleUpdateSurvey({
      ...survey,
      surveys: [updatedSurvey],
    });

    setStableKeys((prev) => [...prev, uuidv4()]);
  };

  const handleRemoveQuestion = (index: number) => {
    const updatedSurvey = [...survey.surveys];
    const updatedQuestions = updatedSurvey[0].questions.filter(
      (_, i) => i !== index,
    );
    updatedSurvey[0].questions = updatedQuestions;
    handleUpdateSurvey({ ...survey, surveys: updatedSurvey });
    setStableKeys((prev) => prev.filter((_, i) => i !== index));
  };

  const handleUpdateQuestion = (
    index: number,
    updated: {
      answerType: AnswerType;
      image_url?: string;
      title: string;
      options?: string[];
      min_label?: string;
      max_label?: string;
      min_value?: number;
      max_value?: number;
      is_multi: boolean;
      is_required?: boolean;
    },
  ) => {
    const updatedSurvey = [...survey.surveys];
    const updatedQuestions = [...updatedSurvey[0].questions];

    let newQuestion: Question;

    if (
      updated.answerType === 'single_choice' ||
      updated.answerType === 'multiple_choice'
    ) {
      newQuestion = {
        answer_type: updated.answerType,
        title: updated.title,
        image_url: updated.image_url,
        options: updated.options || [],
        is_required: updated.is_required || false,
      };
    } else if (updated.answerType === 'checkbox') {
      newQuestion = {
        answer_type: updated.answerType,
        title: updated.title,
        image_url: updated.image_url,
        options: updated.options || [],
        is_multi: updated.is_multi || false,
        is_required: updated.is_required || false,
      };
    } else if (updated.answerType === 'dropdown') {
      newQuestion = {
        answer_type: updated.answerType,
        title: updated.title,
        image_url: updated.image_url,
        options: updated.options || [],
        is_required: updated.is_required || false,
      };
    } else if (updated.answerType === 'linear_scale') {
      newQuestion = {
        answer_type: updated.answerType,
        title: updated.title,
        image_url: updated.image_url,
        min_label: updated.min_label ?? '',
        min_value: updated.min_value ?? 0,
        max_label: updated.max_label ?? '',
        max_value: updated.max_value ?? 0,
        is_required: updated.is_required || false,
      };
    } else {
      newQuestion = {
        answer_type: updated.answerType,
        title: updated.title,
        description: '',
        is_required: updated.is_required || false,
      };
    }

    updatedQuestions[index] = newQuestion;

    updatedSurvey[0].questions = updatedQuestions;

    handleUpdateSurvey({ ...survey, surveys: updatedSurvey });
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
              isMulti={'is_multi' in question ? question.is_multi : false}
              isRequired={
                'is_required' in question ? question.is_required : false
              }
              min={'min_value' in question ? question.min_value : 1}
              max={'max_value' in question ? question.max_value : 10}
              minLabel={'min_label' in question ? question.min_label : ''}
              maxLabel={'max_label' in question ? question.max_label : ''}
              onupdate={(updated) => {
                handleUpdateQuestion(index, {
                  answerType: updated.answerType,
                  title: updated.title,
                  image_url: updated.image_url,
                  options: updated.options ?? [],
                  min_label: updated.min_label ?? '',
                  min_value: updated.min_value ?? 0,
                  max_label: updated.max_label ?? '',
                  max_value: updated.max_value ?? 0,
                  is_multi: updated.is_multi ?? false,
                  is_required: updated.is_required ?? false,
                });
              }}
              onremove={(index: number) => {
                handleRemoveQuestion(index);
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
          onClick={handleAddQuestion}
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
