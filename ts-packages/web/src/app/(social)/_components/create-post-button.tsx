'use client';
import { Edit1 } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { usePostEditorContext } from './post-editor/provider';
import { Button } from '@/components/ui/button';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { useLoggedIn } from '@/hooks/use-user-info';

export default function CreatePostButton({
  team_pk,
  expanded,
}: {
  team_pk?: string;
  expanded?: boolean;
}) {
  const { t } = useTranslation('Home');
  const loggedIn = useLoggedIn();
  const p = usePostEditorContext();
  const createPost = useCreatePostMutation().mutateAsync;
  return (
    <Button
      aria-expanded={expanded}
      aria-label="Create Post"
      variant="rounded_secondary"
      aria-hidden={!loggedIn}
      size="lg"
      className="w-full justify-start max-tablet:aria-[expanded=false]:hidden aria-hidden:hidden"
      onClick={async () => {
        p?.setClose(false);
        const { post_pk } = await createPost({ teamPk: team_pk });
        p?.openPostEditorPopup(post_pk);
      }}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </Button>
  );
}
