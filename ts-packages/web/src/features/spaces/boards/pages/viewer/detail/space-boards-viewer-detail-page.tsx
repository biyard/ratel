import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerDetailController } from './space-boards-viewer-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';
import { useTranslation } from 'react-i18next';
import PostHeader from '../../../components/post-header';
import PostBody from '../../../components/post-body';
import PostComments from '../../../components/post-comments';

export function SpaceBoardsViewerDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsViewerDetailPage: spacePk=${spacePk} postPk=${postPk}`,
  );
  const ctrl = useSpaceBoardsViewerDetailController(spacePk, postPk);
  const { t } = useTranslation('SpaceBoardsEditorDetail');
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
        <PostBody post={ctrl.post} />
        <PostComments
          t={t}
          spacePk={ctrl.spacePk}
          post={ctrl.post}
          isLoggedIn={true}
          expandComment={ctrl.expandComment}
          handleComment={ctrl.handleComment}
          handleReplyToComment={ctrl.handleReplyToComment}
          handleLikeComment={ctrl.handleLikeComment}
        />
      </div>
    </>
  );
}
