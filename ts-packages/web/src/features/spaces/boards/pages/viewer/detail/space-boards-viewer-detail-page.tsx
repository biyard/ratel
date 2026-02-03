import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerDetailController } from './space-boards-viewer-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';
import { useTranslation } from 'react-i18next';
import PostHeader from '../../../components/post-header';
import PostBody from '../../../components/post-body';
import PostComments from '../../../components/post-comments';
import { TimeRangeDisplay } from '../../../components/time-range-display';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { useEffect } from 'react';
import { useLocation } from 'react-router';

export function SpaceBoardsViewerDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsViewerDetailPage: spacePk=${spacePk} postPk=${postPk}`,
  );
  const ctrl = useSpaceBoardsViewerDetailController(spacePk, postPk);
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

  const canActive =
    ctrl.user &&
    (!ctrl.space.anonymous_participation || ctrl.space.participated);

  return (
    <>
      <div className="flex flex-col gap-6 w-full max-tablet:mr-5">
        <PostHeader
          t={t}
          post={ctrl.post}
          handleEditPost={async () => {}}
          handleDeletePost={async () => {}}
          goBack={ctrl.handleBack}
          canDelete={false}
          canEdit={false}
        />
        <TimeRangeDisplay
          startTimestampMillis={ctrl.post?.started_at ?? 0}
          endTimestampMillis={ctrl.post?.ended_at ?? 0}
        />

        <PostBody post={ctrl.post} />
        <PostComments
          t={t}
          spacePk={ctrl.spacePk}
          post={ctrl.post}
          comments={ctrl.comments.get()}
          isFinished={ctrl.space.status === SpaceStatus.Finished}
          isLoggedIn={canActive}
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
