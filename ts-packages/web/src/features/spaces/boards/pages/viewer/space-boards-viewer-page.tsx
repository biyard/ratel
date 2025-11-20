import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsViewerController } from './space-boards-viewer-controller';
import BoardsList from '../../components/boards-list';

export function SpaceBoardsViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpaceBoardsViewerController(spacePk);
  const t = ctrl.t;
  return (
    <>
      <BoardsList
        t={t}
        spacePk={ctrl.spacePk}
        categories={ctrl.categories ?? []}
        posts={ctrl.posts.get() ?? []}
        changeCategory={ctrl.changeCategory}
        handleDetailPage={ctrl.handleDetailPage}
        bookmark={ctrl.bookmark.get()}
        onLoadMore={ctrl.loadMore}
      />
    </>
  );
}
