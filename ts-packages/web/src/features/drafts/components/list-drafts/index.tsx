import { FeedContents, UserBadge } from '@/components/feed-card';
import { Delete2 } from '@/components/icons';
import TimeAgo from '@/components/time-ago';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import PostResponse from '@/features/posts/dto/list-post-response';
import { useCallback, useRef } from 'react';

import { Edit1 } from '@/components/icons';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';

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
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-text-primary">
        {t('no_drafts_label')}
      </div>
    );
  }
  return (
    <Col className="flex-1">
      {drafts.map((post) => (
        <Col
          key={post?.pk}
          className="cursor-pointer pt-5 pb-2.5 bg-card-bg border border-card-enable-border rounded-lg"
          onClick={() => {
            console.log('Move to post edit page - postPk:', post?.pk);
          }}
        >
          <Row className="justify-end px-5 items-center">
            <Row
              className="cursor-pointer w-[21px] h-[21px] hover-bg-white z-100"
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
          <div className="flex flex-row items-center gap-1 w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-text-primary px-5">
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
            contents={post?.html_contents}
            urls={post?.urls ?? []}
          />
        </Col>
      ))}
      <div ref={lastPostRef} />
      {!hasNextPage && <FeedEndMessage msg={t('feed_end_message')} />}
    </Col>
  );
}

export function CreatePostButton({
  onClick,
}: {
  onClick: () => Promise<void>;
}) {
  const { t } = useTranslation('ListDrafts');

  return (
    <Button
      aria-label="Create Post"
      variant="rounded_secondary"
      size="lg"
      className="w-full justify-start"
      onClick={onClick}
    >
      <Edit1 className="w-4 h-4 [&>path]:stroke-text-third" />
      <div className="font-bold text-base/[22px] text-text-third">
        {t('btn_create_post')}
      </div>
    </Button>
  );
}

export function FeedEndMessage({ msg }: { msg: string }) {
  return (
    <div
      className="text-center text-gray-400 my-6"
      aria-label="End of feed message"
    >
      🎉 {msg}
    </div>
  );
}
