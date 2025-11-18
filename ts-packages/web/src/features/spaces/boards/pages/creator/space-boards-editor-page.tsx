import { SpacePathProps } from '@/features/space-path-props';
import { logger } from '@/lib/logger';
import { useSpaceBoardsEditorController } from './space-boards-editor-controller';
import BoardsList from '../../components/boards-list';
import { Button } from '@/components/ui/button';
import { Add } from '@/components/icons';

export function SpaceBoardsEditorPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceBoardsEditorPage: spacePk=${spacePk}`);
  const ctrl = useSpaceBoardsEditorController(spacePk);
  const t = ctrl.t;

  return (
    <div className="w-full">
      <div className="flex flex-row w-full items-end justify-end mb-6">
        <Button
          variant="primary"
          onClick={ctrl.handleCreatePage}
          data-testid="board-btn-create-board"
        >
          <div className="flex flex-row w-fit gap-2">
            <Add className="w-4 h-4 [&>path]:stroke-2 mt-0.5" />
            {t('create_post')}
          </div>
        </Button>
      </div>

      <BoardsList
        t={t}
        spacePk={ctrl.spacePk}
        categories={ctrl.categories ?? []}
        posts={ctrl.posts.get() ?? []}
        changeCategory={ctrl.changeCategory}
        handleDetailPage={ctrl.handleDetailPage}
      />
    </div>
  );
}
