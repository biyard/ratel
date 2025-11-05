import { logger } from '@/lib/logger';
import { useSpaceBoardsEditorDetailController } from './space-boards-editor-detail-controller';
import { SpacePostPathProps } from '../../space-post-path-props';
import PostHeader from '../../../components/post-header';
import { useTranslation } from 'react-i18next';
import PostBody from '../../../components/post-body';
import PostComments from '../../../components/post-comments';

export function SpaceBoardsEditorDetailPage({
  spacePk,
  postPk,
}: SpacePostPathProps) {
  logger.debug(
    `SpaceBoardsEditorDetailPage: spacePk=${spacePk} postPk: ${postPk}`,
  );
  const ctrl = useSpaceBoardsEditorDetailController(spacePk, postPk);
  const { t } = useTranslation('SpaceBoardsEditorDetail');

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
