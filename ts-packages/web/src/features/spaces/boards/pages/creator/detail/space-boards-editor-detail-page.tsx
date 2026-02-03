import { logger } from '@/lib/logger';
import { useSpaceBoardsEditorDetailController } from './space-boards-editor-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';
import PostHeader from '../../../components/post-header';
import { useTranslation } from 'react-i18next';
import PostBody from '../../../components/post-body';
import PostComments from '../../../components/post-comments';
import { TimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { useEffect } from 'react';
import { useLocation } from 'react-router';
// import { TimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';

export function SpaceBoardsEditorDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsEditorDetailPage: spacePk=${spacePk} postPk: ${postPk}`,
  );
  const ctrl = useSpaceBoardsEditorDetailController(spacePk, postPk);
  const { t } = useTranslation('SpaceBoardsEditorDetail');
  const location = useLocation();

  useEffect(() => {
    if (location.hash === '#comments') {
      setTimeout(() => {
        const element = document.getElementById('comments');
        element?.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }, 100);
    }
  }, [location.hash]);

  return (
    <>
      <div className="flex flex-col gap-6 w-full max-tablet:mr-[20px]">
        <PostHeader
          t={t}
          post={ctrl.post}
          handleEditPost={ctrl.handleEditPost}
          handleDeletePost={ctrl.handleDeletePost}
          goBack={ctrl.handleBack}
          canDelete={true}
          canEdit={true}
        />
        <TimeRangeSetting
          canEdit={false}
          onChange={() => {}}
          startTimestampMillis={ctrl.post?.started_at ?? 0}
          endTimestampMillis={ctrl.post?.ended_at ?? 0}
          alwaysEdit={false}
          className="justify-end"
        />
        <PostBody post={ctrl.post} />
        <PostComments
          t={t}
          spacePk={ctrl.spacePk}
          post={ctrl.post}
          comments={ctrl.comments.get()}
          isFinished={ctrl.space.status === SpaceStatus.Finished}
          isLoggedIn={true}
          expandComment={ctrl.expandComment}
          handleCommentDelete={ctrl.handleDeleteComment}
          handleCommentUpdate={ctrl.handleUpdateComment}
          handleComment={ctrl.handleComment}
          handleReplyToComment={ctrl.handleReplyToComment}
          handleLikeComment={ctrl.handleLikeComment}
          hasPrevPage={ctrl.hasPrevPage()}
          hasNextPage={ctrl.hasNextPage()}
          onPrevPage={ctrl.handlePrevCommentsPage}
          onNextPage={ctrl.handleNextCommentsPage}
        />
      </div>
    </>
  );
}
