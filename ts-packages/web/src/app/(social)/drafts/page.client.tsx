'use client';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import React, { useCallback, useRef } from 'react';
import { Col } from '@/components/ui/col';
import { FeedStatus } from '@/lib/api/models/feeds';
import { usePostEditorContext } from '../_components/post-editor';
import CreatePostButton from '../_components/create-post-button';
import { Row } from '@/components/ui/row';
import { FeedContents, UserBadge } from '@/components/feed-card';
import { UserType } from '@/lib/api/models/user';
import TimeAgo from '@/components/time-ago';
import { Delete2 } from '@/components/icons';
import { usePostInfiniteQuery, usePostMutation } from '@/hooks/use-post';

export default function DraftPage() {
  const { data: user } = useSuspenseUserInfo();
  const user_id = user?.id || 0;
  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = usePostInfiniteQuery(user_id, FeedStatus.Draft);

  const { openPostEditorPopup } = usePostEditorContext();
  const { delete: deleteDraft } = usePostMutation(user_id, FeedStatus.Draft);
  const removeDraft = async (post_id: number) => {
    await deleteDraft.mutateAsync(post_id);
  };
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

  if (drafts.pages.length === 0) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        No drafts available
      </div>
    );
  }
  const flattedDrafts = drafts?.pages.flatMap((page) => page) ?? [];
  return (
    <div className="flex flex-1 relative">
      <div className="flex-1 flex max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedDrafts.map((post) => (
            <Col
              key={post.id}
              className="cursor-pointer pt-5 pb-2.5 bg-component-bg rounded-lg"
              onClick={async (evt) => {
                await openPostEditorPopup(post.id);
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

                    await removeDraft(post.id);
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
                <div className="text-sm font-normal">(Draft)</div>
                <div className="font-normal">{post.title}</div>
              </div>
              <Row className="justify-between items-center px-5">
                <UserBadge
                  profile_url={user.profile_url ?? ''}
                  name={user.username}
                  author_type={UserType.Individual}
                />
                <TimeAgo timestamp={post.updated_at} />
              </Row>
              <Row className="justify-between px-5"></Row>
              <FeedContents contents={post.html_contents} url={post.url} />
            </Col>
          ))}
          <div ref={lastPostRef} />
        </Col>
      </div>

      <div
        className={`z-50 max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton />
      </div>
    </div>
  );
}
