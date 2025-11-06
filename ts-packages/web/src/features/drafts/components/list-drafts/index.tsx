import { FeedContents, UserBadge } from '@/components/feed-card';
import { Delete2 } from '@/components/icons';
import TimeAgo from '@/components/time-ago';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import PostResponse from '@/features/posts/dto/list-post-response';
import { useCallback, useRef } from 'react';

import { Edit1 } from '@/components/icons';
import { buttonVariants } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { Link, useNavigate } from 'react-router';
import { route } from '@/route';
import { cn } from '@/lib/utils';

export default function ListDrafts({
  drafts,
  fetchNextPage,
  hasNextPage,
  isFetchingNextPage,
  onDelete,
}: {
  drafts: PostResponse[];
  fetchNextPage: () => void;
  hasNextPage: boolean;
  isFetchingNextPage: boolean;
  onDelete: (postPk: string) => void;
}) {
  const { t } = useTranslation('ListDrafts');
  const observer = useRef<IntersectionObserver | null>(null);
  const navigate = useNavigate();
  const lastPostRef = useCallback(
    (node: HTMLDivElement) => {
      if (isFetchingNextPage) return;
      if (observer.current) observer.current.disconnect();
      observer.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && hasNextPage) {
          fetchNextPage();
        }
      });
      if (node) observer.current.observe(node);
    },
    [isFetchingNextPage, fetchNextPage, hasNextPage],
  );

  if (drafts.length === 0) {
    return (
      <div className="flex flex-row justify-start items-center w-full text-base font-medium border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px] text-text-primary">
        {t('no_drafts_label')}
      </div>
    );
  }
  return (
    <Col className="flex-1">
      {drafts.map((post) => (
        <Col
          key={post?.pk}
          className="pt-5 pb-2.5 rounded-lg border cursor-pointer bg-card-bg border-card-enable-border"
          onClick={() => {
            navigate(route.newPost(post.pk));
          }}
        >
          <Row className="justify-end items-center px-5">
            <Row
              className="cursor-pointer hover-bg-white w-[21px] h-[21px] z-100"
              onClick={async (e) => {
                e.preventDefault();
                e.stopPropagation();

                await onDelete(post.pk);
              }}
            >
              {
                <Delete2
                  width={24}
                  height={24}
                  className="[&>path]:stroke-neutral-500"
                />
              }
            </Row>
          </Row>
          <div className="flex flex-row gap-1 items-center px-5 w-full font-bold align-middle line-clamp-2 text-xl/[25px] tracking-[0.5px] text-text-primary">
            <div className="text-sm font-normal">(Draft)</div>
            <div className="font-normal">{post?.title}</div>
          </div>
          <Row className="justify-between items-center px-5">
            <UserBadge
              profile_url={post.author_profile_url ?? ''}
              name={post.author_display_name}
              author_type={post.author_type}
            />
            <TimeAgo timestamp={post?.updated_at} />
          </Row>
          <Row className="justify-between px-5"></Row>
          <FeedContents
            contents={post?.html_contents.slice(0, 200) ?? ''}
            urls={post?.urls ?? []}
          />
        </Col>
      ))}
      <div ref={lastPostRef} />
      {!hasNextPage && <FeedEndMessage msg={t('feed_end_message')} />}
    </Col>
  );
}

export function CreatePostButton({ teamPk }: { teamPk?: string | undefined }) {
  const { t } = useTranslation('ListDrafts');

  const baseClass = buttonVariants({
    variant: 'rounded_secondary',
    size: 'lg',
  });

  return (
    <Link
      to={route.newPost(undefined, teamPk)}
      aria-label="Create Post"
      className={cn(baseClass, 'justify-start w-full')}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('btn_create_post')}
      </div>
    </Link>
  );
}

export function FeedEndMessage({ msg }: { msg: string }) {
  return (
    <div
      className="my-6 text-center text-gray-400"
      aria-label="End of feed message"
    >
      🎉 {msg}
    </div>
  );
}
