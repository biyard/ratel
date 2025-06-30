'use client';

import React from 'react';
import SpaceHeader from '../../_components/space_header';
import SpaceSurvey from './space_survey';
import { Question } from '@/lib/api/models/survey';
import { AnswerType } from './question/answer_type_select';
import { useDeliberationSpaceContext } from '../provider.client';
// import { Poll } from '../page.client';

export default function PollPage() {
  const {
    isEdit,
    title,
    setTitle,
    startedAt: startDate,
    endedAt: endDate,
    survey,
    setSurvey,
    answer,
    status,
    userType,
    proposerImage,
    proposerName,
    createdAt,
    handleGoBack,
    handleSetAnswers: setAnswers,
    handleSetEndDate: setEndDate,
    handleSetStartDate: setStartDate,
    handleSend: onsend,
  } = useDeliberationSpaceContext();

  const questions =
    survey.surveys.length != 0 ? survey.surveys[0].questions : [];
  return (
    <div className="flex flex-col w-full">
      <SpaceHeader
        isEdit={isEdit}
        title={title}
        status={status}
        userType={userType}
        proposerImage={proposerImage}
        proposerName={proposerName}
        createdAt={createdAt}
        onback={handleGoBack}
        setTitle={(title: string) => {
          setTitle(title);
        }}
      />

      <div className="flex flex-col mt-[25px] gap-2.5">
        <SpaceSurvey
          isEdit={isEdit}
          status={status}
          questions={questions}
          startDate={startDate}
          endDate={endDate}
          answer={answer}
          setAnswers={setAnswers}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
          onsend={onsend}
          onadd={(question: Question) => {
            if (survey.surveys.length === 0) {
              setSurvey({
                ...survey,
                surveys: [
                  {
                    started_at: 0,
                    ended_at: 10000000000,
                    questions: [question],
                  },
                ],
              });
            } else {
              const updatedSurveys = [...survey.surveys];
              updatedSurveys[0] = {
                ...updatedSurveys[0],
                questions: [...updatedSurveys[0].questions, question],
              };

              setSurvey({
                ...survey,
                surveys: updatedSurveys,
              });
            }
          }}
          onupdate={(
            index: number,
            updated: {
              answerType: AnswerType;
              title: string;
              options?: string[];
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
                options: updated.options || [],
              };
            } else {
              newQuestion = {
                answer_type: updated.answerType,
                title: updated.title,
                description: '',
              };
            }

            updatedQuestions[index] = newQuestion;
            updatedSurvey[0].questions = updatedQuestions;
            setSurvey({ ...survey, surveys: updatedSurvey });
          }}
          onremove={(index: number) => {
            const updatedSurvey = [...survey.surveys];
            const updatedQuestions = updatedSurvey[0].questions.filter(
              (_, i) => i !== index,
            );
            updatedSurvey[0].questions = updatedQuestions;
            setSurvey({ ...survey, surveys: updatedSurvey });
          }}
        />
      </div>
    </div>
  );
}
