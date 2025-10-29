'use client';
import { Edit1 } from '@/components/icons';
import { useTranslation } from 'react-i18next';
import { buttonVariants } from '@/components/ui/button';
import { useLoggedIn } from '@/hooks/use-user-info';
import { Link } from 'react-router';
import { cn } from '@/lib/utils';
import { route } from '@/route';

export default function CreatePostButton({
  team_pk,
  expanded,
}: {
  team_pk?: string;
  expanded?: boolean;
}) {
  const { t } = useTranslation('Home');
  const loggedIn = useLoggedIn();

  const base = buttonVariants({
    variant: 'rounded_secondary',
    size: 'lg',
  });

  return (
    <Link
      to={route.createPost(undefined, team_pk)}
      aria-expanded={expanded}
      aria-label="Create Post"
      aria-hidden={!loggedIn}
      className={cn(
        base,
        'w-full justify-start max-tablet:aria-[expanded=false]:hidden aria-hidden:hidden',
      )}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('create_post')}
      </div>
    </Link>
  );
}
