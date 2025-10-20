'use client';
import { Edit1 } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { usePostEditorContext } from '@/app/(social)/_components/post-editor';
import { createPost } from '@/lib/api/ratel/posts.v3';

export default function CreatePostButton({ teamPk }: { teamPk: string }) {
  const { t } = useTranslation('Team');
  const p = usePostEditorContext();

  if (!p || !teamPk) return null;

  const { openPostEditorPopup, setClose } = p;

  return (
    <div
      className="cursor-pointer flex flex-row w-full justify-start items-center gap-1 bg-create-button-bg rounded-[100px] px-4 py-3 mb-[10px]"
      onClick={async () => {
        setClose(false);
        const { post_pk } = await createPost(teamPk);
        openPostEditorPopup(post_pk);
      }}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </div>
  );
}
