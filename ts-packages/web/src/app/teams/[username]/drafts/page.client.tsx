'use client';
import React, { useCallback } from 'react';
import { Col } from '@/components/ui/col';
import { FeedStatus } from '@/lib/api/models/feeds';
import { Row } from '@/components/ui/row';
import { FeedContents, UserBadge } from '@/components/feed-card';
import { UserType } from '@/lib/api/models/user';
import TimeAgo from '@/components/time-ago';
import { Delete2 } from '@/components/icons';
import { useTeamByUsername } from '../../_hooks/use-team';
import CreatePostButton from '../_components/create-post-button';
import { useTranslations } from 'next-intl';
import { usePostEditorContext } from '@/app/(social)/_components/post-editor';
import useInfiniteFeeds from '@/hooks/feeds/use-feeds-infinite-query';
import { useObserver } from '@/hooks/use-observer';
import { useDeletePostMutation } from '@/hooks/feeds/use-delete-post-mutation';

// Duplicated from src/app/%28social%29/drafts/page.client.tsx
// Should be moved to a shared location

export default function TeamDraftPage({ username }: { username: string }) {
  const t = useTranslations('Team');
  const { data: team } = useTeamByUsername(username);

  const p = usePostEditorContext();

  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteFeeds();

  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);
  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  const { mutateAsync: handleRemoveDraft } = useDeletePostMutation(
    team.username,
    FeedStatus.Draft,
  );

  if (drafts.pages.length === 0) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        {t('no_drafts_available')}
      </div>
    );
  }
  const flattedDrafts = drafts?.pages.flatMap((page) => page.items) ?? [];

  return (
    <div className="flex flex-1 relative">
      <div className="flex-1 flex max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedDrafts.map((post) => (
            <Col
              key={post.pk}
              className="cursor-pointer pt-5 pb-2.5 bg-card-bg border border-card-border rounded-lg"
              onClick={async (evt) => {
                await p?.openPostEditorPopup(post.pk);
                evt.preventDefault();
                evt.stopPropagation();
              }}
            >
              <Row className="justify-end px-5 items-center">
                {/* <Row>
                        <IndustryTag industry={'CRYPTO'} />
                      </Row> */}
                <Row
                  className="cursor-pointer w-[21px] h-[21px]"
                  onClick={async (e) => {
                    e.preventDefault();
                    e.stopPropagation();

                    await handleRemoveDraft(post.pk);
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
              <div className="flex flex-row items-center gap-1 w-full line-clamp-2 font-bold text-xl/[25px] tracking-[0.5px] align-middle text-white px-5">
                <div className="text-sm font-normal text-text-primary">
                  (Draft)
                </div>
                <div className="font-normal text-text-primary">
                  {post.title}
                </div>
              </div>
              <Row className="justify-between items-center px-5 text-text-primary">
                <UserBadge
                  profile_url={team.profile_url ?? ''}
                  name={team.nickname}
                  author_type={UserType.Team}
                />
                <TimeAgo timestamp={post.updated_at} />
              </Row>
              <Row className="justify-between px-5"></Row>
              <FeedContents contents={post.html_contents} urls={post.urls} />
            </Col>
          ))}
          <div ref={observerRef} />
        </Col>
      </div>

      <div className="w-80 pl-4 max-tablet:!hidden">
        <CreatePostButton />
      </div>
    </div>
  );
}
