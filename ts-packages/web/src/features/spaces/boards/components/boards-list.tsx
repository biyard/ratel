import { PostEditor } from '@/features/posts/components/post-editor';
import { useEffect, useMemo, useState } from 'react';
import DOMPurify from 'dompurify';
import { getTimeAgo } from '@/lib/time-utils';
import { SpacePostResponse } from '../types/space-post-response';

import { Col } from '@/components/ui/col';
import { UserBadge } from '@/components/feed-card';
import { UserType } from '@/lib/api/ratel/users.v3';
import { Button } from '@/components/ui/button';
import { TFunction } from 'i18next';

export type BoardsListProps = {
  t: TFunction<'SpaceBoardsEditor', undefined>;
  spacePk: string;
  categories: string[];
  posts: SpacePostResponse[];
  changeCategory: (categoryName: string) => void;
};

export default function BoardsList({
  t,
  categories,
  posts,
  changeCategory,
}: BoardsListProps) {
  const [selected, setSelected] = useState<string | null>(null);

  useEffect(() => {
    changeCategory(selected ?? '');
  }, [selected]);

  const sortedPosts = useMemo(() => {
    const list = (posts ?? []).slice();
    list.sort((a, b) => {
      const A =
        typeof a.created_at === 'number'
          ? a.created_at
          : new Date(a.created_at ?? 0).getTime();
      const B =
        typeof b.created_at === 'number'
          ? b.created_at
          : new Date(b.created_at ?? 0).getTime();
      return B - A;
    });
    return list;
  }, [posts]);

  return (
    <div className="flex flex-col w-full">
      <div className="mb-4">
        <div className="flex flex-wrap gap-2 mt-[10px]">
          <Button
            type="button"
            onClick={() => setSelected(null)}
            className={[
              'px-3 py-1.5 rounded-lg text-sm transition-colors border',
              selected === null
                ? 'bg-primary text-black border border-none'
                : 'bg-neutral-700 light:bg-neutral-500 hover:bg-neutral-700/80 hover:light:bg-neutral-500/80 text-neutral-200 border border-none',
            ].join(' ')}
          >
            {t('total')}
          </Button>
          {categories.map((c) => {
            const active = selected === c;
            return (
              <button
                key={c}
                type="button"
                onClick={() => setSelected(active ? null : c)}
                className={[
                  'px-3 py-1.5 rounded-lg text-sm transition-colors border',
                  active
                    ? 'bg-primary text-black border border-none'
                    : 'bg-neutral-700 light:bg-neutral-500 hover:bg-neutral-700/80 hover:light:bg-neutral-500/80 text-neutral-200 border border-none',
                ].join(' ')}
              >
                {c}
              </button>
            );
          })}
        </div>
      </div>

      <Col className="grid gap-4">
        {sortedPosts.map((p) => (
          <div
            key={p.pk}
            className="w-full bg-card-bg-secondary border-card-enable-border rounded-[10px] py-[20px]"
          >
            <div className="flex flex-col w-full gap-3">
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-center flex-wrap">
                  <h3 className="text-base font-semibold text-text-primary px-5">
                    {p.title}
                  </h3>
                  {p.category_name && (
                    <span className="inline-flex items-center rounded-md border border-neutral-700 bg-neutral-800 light:bg-neutral-500 light:border-0 px-2 py-0.5 text-xs text-neutral-200">
                      {p.category_name}
                    </span>
                  )}
                </div>
              </div>

              {p.html_contents && (
                <div className="break-all text-desc-text">
                  <PostEditor
                    editable={false}
                    showToolbar={false}
                    content={DOMPurify.sanitize(p.html_contents)}
                    minHeight="50px"
                    maxHeight="200px"
                    url={p.urls.length == 0 ? '' : p.urls[0]}
                  />
                </div>
              )}

              <div className="flex items-center gap-3 text-xs text-neutral-400 px-5">
                <UserBadge
                  profile_url={p.author_profile_url}
                  name={p.author_username}
                  author_type={UserType.Individual}
                />
                {p.created_at && (
                  <span className="inline-flex items-center gap-1 mt-1">
                    <span className="i-lucide-calendar text-[14px]" />
                    {getTimeAgo(
                      (typeof p.created_at === 'number'
                        ? p.created_at
                        : new Date(p.created_at ?? 0).getTime()) / 1000,
                    )}
                  </span>
                )}
              </div>
            </div>
          </div>
        ))}

        {!sortedPosts.length && (
          <div className="rounded-2xl border border-neutral-800 bg-neutral-900 p-10 text-center text-neutral-400">
            {t('no_post')}
          </div>
        )}
      </Col>
    </div>
  );
}
