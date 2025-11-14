import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerDetailController } from './space-boards-viewer-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';
import { useTranslation } from 'react-i18next';
import PostHeader from '../../../components/post-header';
import PostBody from '../../../components/post-body';
import PostComments from '../../../components/post-comments';
import { TimeRangeSetting } from '@/features/spaces/polls/components/time-range-setting';

export function SpaceBoardsViewerDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsViewerDetailPage: spacePk=${spacePk} postPk=${postPk}`,
  );
  const ctrl = useSpaceBoardsViewerDetailController(spacePk, postPk);
  const { t } = useTranslation('SpaceBoardsEditorDetail');

  const canActive =
    ctrl.user &&
    (!ctrl.space.anonymous_participation || ctrl.space.participated);

  return (
    <>
      <div className="flex flex-col gap-6 w-full max-tablet:mr-[20px]">
        <PostHeader
          t={t}
          post={ctrl.post}
          handleEditPost={async () => {}}
          handleDeletePost={async () => {}}
          goBack={ctrl.handleBack}
          canDelete={false}
          canEdit={false}
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
          isLoggedIn={canActive}
          expandComment={ctrl.expandComment}
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
