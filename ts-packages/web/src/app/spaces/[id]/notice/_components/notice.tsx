'use client';

import { useNoticeSpaceContext } from '../provider.client';
import { useSpaceByIdContext } from '../../providers.client';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import SpaceContents from '../../_components/space-contents';
import QuizBuilderUI, { Question } from './quiz-builder-ui';

type NoticePageProps = {
  onSubmitQuiz?: (questions: Question[]) => Promise<void>;
};

const NoticePage: React.FC<NoticePageProps> = ({ onSubmitQuiz }) => {
  const {
    isEdit,
    htmlContent,
    setHtmlContent,
    quizQuestions,
    setQuizQuestions,
    space,
  } = useNoticeSpaceContext();

  const { spaceId } = useSpaceByIdContext();
  const { data: userInfo } = useUserInfo();

  const isOwner = userInfo?.username === space?.owner_username;

  const handleQuestionsChange = (updatedQuestions: Question[]) => {
    setQuizQuestions(updatedQuestions);
  };

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-col gap-2">
        <SpaceContents
          isEdit={isEdit}
          htmlContents={htmlContent}
          setContents={setHtmlContent}
        />
        <QuizBuilderUI
          isEditMode={isEdit}
          questions={quizQuestions}
          onQuestionsChange={handleQuestionsChange}
          onSubmitQuiz={onSubmitQuiz}
          spaceId={spaceId}
          userId={userInfo?.pk}
          isOwner={isOwner}
          spaceStatus={space?.status}
        />
      </div>
    </div>
  );
};

export default NoticePage;
