'use client';

import React from 'react';
import { useNoticeSpaceContext } from '../provider.client';
import { useSpaceByIdContext } from '../../providers.client';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import SpaceContents from '../../_components/space_contents';
import QuizBuilderUI, { Question } from './quiz-builder-ui';

export default function NoticePage() {
  const {
    isEdit,
    htmlContent,
    setHtmlContent,
    quizQuestions,
    setQuizQuestions,
    handleSubmitQuiz,
    space,
  } = useNoticeSpaceContext();
  const { spaceId } = useSpaceByIdContext();
  const { data: userInfo } = useUserInfo();

  // Check if current user is the space owner
  const isOwner = userInfo?.id === space?.owner_id;

  // Handler for quiz data changes
  const handleQuestionsChange = (updatedQuestions: Question[]) => {
    setQuizQuestions(updatedQuestions);
    // Note: The questions will be saved when the user saves the space
    console.log('Quiz data updated:', updatedQuestions);
  };

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2">
        {/* Main Content Area with SpaceContents for editing */}
        <SpaceContents
          isEdit={isEdit}
          htmlContents={htmlContent}
          setContents={(content) => setHtmlContent(content)}
        />

        {/* Quiz Section - Now uses integrated component */}
        <QuizBuilderUI
          isEditMode={isEdit}
          questions={quizQuestions}
          onQuestionsChange={handleQuestionsChange}
          onSubmitQuiz={handleSubmitQuiz}
          spaceId={spaceId}
          userId={userInfo?.id}
          isOwner={isOwner}
          spaceStatus={space?.status}
        />
      </div>
    </div>
  );
}
