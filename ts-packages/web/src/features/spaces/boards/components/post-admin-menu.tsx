import { TFunction } from 'i18next';
import { Trash2, Edit } from 'lucide-react';

import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Extra } from '@/components/icons';
import { logger } from '@/lib/logger';

export type PostAdminMenuProps = {
  t: TFunction<'SpaceBoardsEditorDetail', undefined>;
  canDelete: boolean;
  canEdit: boolean;
  handleEditPost: () => Promise<void>;
  handleDeletePost: () => Promise<void>;
};

export default function PostAdminMenu(props: PostAdminMenuProps) {
  const { t, canDelete, canEdit, handleEditPost } = props;

  if (!canDelete && !canEdit) return null;

  logger.debug('Rendering ThreadAdminMenu:', props);

  return (
    <div aria-label="Post Admin Menu" className="flex items-center space-x-2.5">
      {canEdit && (
        <>
          <Button
            aria-label="Edit Post"
            variant="rounded_secondary"
            className="rounded-md max-tablet:hidden text-sm px-3 py-1.5 text-button-text bg-button-bg hover:bg-button-bg/80"
            onClick={handleEditPost}
          >
            <Edit className="!size-5" />
            {t('edit')}
          </Button>
        </>
      )}

      {canDelete && (
        <DesktopContextMenus className="block max-tablet:hidden" {...props} />
      )}

      <MobileContextMenus className="hidden max-tablet:block" {...props} />
    </div>
  );
}

export function DesktopContextMenus({
  canDelete,
  handleDeletePost,
  t,
  className,
}: PostAdminMenuProps & React.HTMLAttributes<HTMLDivElement>) {
  if (!canDelete) return null;

  return (
    <div className={className}>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <button
            className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
            aria-haspopup="true"
            aria-label="Post options for desktop"
          >
            <Extra className="size-6 text-gray-400" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          className="w-40 transition ease-out duration-100"
        >
          <DropdownMenuItem>
            <button
              aria-label="Delete Post"
              onClick={handleDeletePost}
              className="flex items-center w-full px-4 max-tablet:justify-start max-tablet:gap-1 max-tablet:hover:bg-transparent max-tablet:px-0 py-2 text-sm text-red-400 cursor-pointer"
            >
              <Trash2 className="w-4 h-4" />
              {t('delete')}
            </button>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}

export function MobileContextMenus({
  canEdit,
  canDelete,
  handleEditPost,
  handleDeletePost,
  t,
  className,
}: PostAdminMenuProps & React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={className}>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <button
            className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
            aria-haspopup="true"
            aria-label="Post options for mobile"
          >
            <Extra className="size-6 text-gray-400" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          className="w-40 transition ease-out duration-100"
        >
          {canEdit && (
            <>
              <DropdownMenuItem>
                <button
                  onClick={handleEditPost}
                  className="flex items-center max-tablet:justify-start gap-1 max-tablet:hover:bg-transparent w-full py-2 text-sm text-text-primary cursor-pointer"
                >
                  <Edit className="w-4 h-4 [&>path]:stroke-text-primary" />
                  {t('edit')}
                </button>
              </DropdownMenuItem>
            </>
          )}

          {canDelete && (
            <>
              <DropdownMenuItem>
                <button
                  onClick={handleDeletePost}
                  className="flex items-center w-full px-4 max-tablet:justify-start max-tablet:gap-1 max-tablet:hover:bg-transparent max-tablet:px-0 py-2 text-sm text-red-400 cursor-pointer"
                >
                  <Trash2 className="w-4 h-4" />
                  {t('delete')}
                </button>
              </DropdownMenuItem>
            </>
          )}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
