import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useCallback, useRef } from 'react';
import { Col } from '@/components/ui/col';
import { FeedStatus } from '@/features/posts/types/post';
import { usePostEditorContext } from '../_components/post-editor';
import CreatePostButton from '../_components/create-post-button';
import { Row } from '@/components/ui/row';
import { FeedContents, UserBadge } from '@/components/feed-card';
import { UserType } from '@/lib/api/ratel/users.v3';

import TimeAgo from '@/components/time-ago';
import { Delete2 } from '@/components/icons';
import { useDeletePostMutation } from '@/features/posts/hooks/use-delete-post-mutation';
import useInfiniteMyDrafts from './_hooks/use-my-drafts';
import { Link } from 'react-router';
import { route } from '@/route';

export default function MyDraftPage() {
  const { data: user } = useSuspenseUserInfo();
  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteMyDrafts();

  const p = usePostEditorContext();
  const username = user?.username || '';

  const { mutateAsync: handleRemoveDraft } = useDeletePostMutation(
    username,
    FeedStatus.Draft,
  );

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

  if (!p) return null;
  if (!user) return null;

  const { openPostEditorPopup } = p;

  if (drafts.pages.length === 0) {
    return (
      <div className="flex flex-row justify-start items-center w-full text-base font-medium border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px] text-text-primary">
        No drafts available
      </div>
    );
  }
  const flattedDrafts = drafts?.pages.flatMap((page) => page.items) ?? [];
  return (
    <div className="flex relative flex-1">
      <div className="flex flex-1 max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedDrafts.map((post) => (
            <Link key={`draft-${post!.pk}`} to={route.createPost(post!.pk)}>
              <Col
                key={post?.pk}
                className="pt-5 pb-2.5 rounded-lg border cursor-pointer bg-card-bg border-card-enable-border"
              >
                <Row className="justify-end items-center px-5">
                  {/* <Row>
                  <IndustryTag industry={'CRYPTO'} />
                </Row> */}
                  <Row
                    className="cursor-pointer hover-bg-white w-[21px] h-[21px] z-100"
                    onClick={async (e) => {
                      e.preventDefault();
                      e.stopPropagation();

                      await handleRemoveDraft(post?.pk);
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
                    profile_url={user.profile_url ?? ''}
                    name={user.username}
                    author_type={UserType.Individual}
                  />
                  <TimeAgo timestamp={post?.updated_at} />
                </Row>
                <Row className="justify-between px-5"></Row>
                <FeedContents
                  contents={post?.html_contents.slice(0, 300) ?? ''}
                  urls={post?.urls ?? []}
                />
              </Col>
            </Link>
          ))}
          <div ref={lastPostRef} />
        </Col>
      </div>

      <div
        className={`h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton />
      </div>
    </div>
  );
}
