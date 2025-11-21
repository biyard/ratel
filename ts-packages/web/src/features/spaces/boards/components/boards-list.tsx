import { PostEditor } from '@/features/posts/components/post-editor';
import { useEffect, useRef, useState } from 'react';
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
  handleDetailPage: (postPk: string) => void;

  bookmark: string | null | undefined;
  onLoadMore: (categoryName: string) => Promise<void> | void;
};

export default function BoardsList({
  t,
  categories,
  posts,
  changeCategory,
  handleDetailPage,
  bookmark,
  onLoadMore,
}: BoardsListProps) {
  const [selected, setSelected] = useState<string | null>(null);
  const [loadingMore, setLoadingMore] = useState(false);
  const bottomRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    changeCategory(selected ?? '');
  }, [selected]);

  const handleLoadMore = async () => {
    if (!bookmark || loadingMore) return;

    setLoadingMore(true);
    try {
      await onLoadMore(selected ?? '');
      requestAnimationFrame(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth', block: 'end' });
      });
    } finally {
      setLoadingMore(false);
    }
  };

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
        {posts.map((p) => (
          <div
            key={p.pk}
            className="w-full cursor-pointer bg-card-bg-secondary border-card-enable-border rounded-[10px] py-[20px]"
            data-testid="board-post-item"
            onClick={() => {
              handleDetailPage(p.pk);
            }}
          >
            <div className="flex flex-col gap-3 w-full">
              <div className="flex gap-3 justify-between items-start">
                <div className="flex flex-wrap items-center">
                  <h3 className="px-5 text-base font-semibold text-text-primary">
                    {p.title}
                  </h3>

                  <span className="inline-flex items-center py-0.5 px-2 text-xs rounded-md border border-neutral-700 bg-neutral-800 light:bg-neutral-500 light:border-0 text-neutral-200">
                    <span className="i-lucide-message-circle text-[13px]" />
                    {p.number_of_comments} {t('response')}
                  </span>

                  {p.category_name && (
                    <span className="inline-flex items-center py-0.5 px-2 ml-2 text-xs rounded-md border border-neutral-700 bg-neutral-800 light:bg-neutral-500 light:border-0 text-neutral-200">
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

              <div className="flex gap-3 items-center px-5 text-xs text-neutral-400">
                <UserBadge
                  profile_url={p.author_profile_url}
                  name={p.author_username}
                  author_type={UserType.Individual}
                />
                {p.created_at && (
                  <span className="inline-flex gap-1 items-center mt-1">
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

        {!posts.length && (
          <div className="p-10 text-center rounded-2xl border border-neutral-800 bg-neutral-900 text-neutral-400">
            {t('no_post')}
          </div>
        )}

        {posts.length > 0 && (
          <div className="flex justify-center py-6">
            {bookmark ? (
              <Button
                variant="text"
                disabled={loadingMore}
                onClick={handleLoadMore}
                className="py-2 px-6 rounded-lg light:hover:text-neutral-300"
              >
                {loadingMore ? 'Loading...' : 'More'}
              </Button>
            ) : (
              <></>
            )}
          </div>
        )}

        <div ref={bottomRef} />
      </Col>
    </div>
  );
}
