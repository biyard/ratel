'use client';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import React from 'react';
import { usePostByUserId } from '../_hooks/use-posts';
import { Col } from '@/components/ui/col';
import { logger } from '@/lib/logger';
import { FeedStatus } from '@/lib/api/models/feeds';
import { usePostDraft } from '../_components/create-post';
import CreatePostButton from '../_components/create-post-button';
import { Row } from '@/components/ui/row';
import { FeedContents, IndustryTag, UserBadge } from '@/components/feed-card';
import { UserType } from '@/lib/api/models/user';
import TimeAgo from '@/components/time-ago';
import { Delete2 } from '@/components/icons';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { removeDraftRequest } from '@/lib/api/models/feeds/update-draft-request';
import { checkString } from '@/lib/string-filter-utils';

export default function MyPostsPage() {
  const { post } = useApiCall();
  const { data: user } = useSuspenseUserInfo();
  const user_id = user?.id || 0;
  const posts = usePostByUserId(user_id, 1, 20, FeedStatus.Draft);
  const data = posts.data;
  logger.debug('query response of posts', data);
  const { loadDraft } = usePostDraft();

  const drafts = data?.items.map((item) => ({
    id: item.id,
    title: item.title!,
    updated_at: item.updated_at,
    contents: item.html_contents,
    url: item.url,
  }));

  const removeDraft = async (draft_id: number) => {
    const res = await post(
      ratelApi.feeds.removeDraft(draft_id),
      removeDraftRequest(),
    );
    if (res) {
      posts.refetch?.();
    }
  };

  return (
    <div className="flex flex-1 relative">
      <div className="flex-1 flex max-mobile:px-[10px]">
        {drafts?.length != 0 ? (
          <Col className="flex-1">
            {drafts
              ?.filter(
                (d) =>
                  !(
                    checkString(d.title) ||
                    checkString(d.contents) ||
                    checkString(user.username)
                  ),
              )
              .map((props) => (
                <Col
                  key={props.id}
                  className="cursor-pointer pt-5 pb-2.5 bg-component-bg rounded-lg"
                  onClick={(evt) => {
                    loadDraft(props.id);
                    evt.preventDefault();
                    evt.stopPropagation();
                  }}
                >
                  <Row className="justify-between px-5 items-center">
                    <Row>
                      <IndustryTag industry={'CRYPTO'} />
                    </Row>
                    <Row
                      className="cursor-pointer w-[21px] h-[21px]"
                      onClick={async (e) => {
                        e.preventDefault();
                        e.stopPropagation();

                        await removeDraft(props.id);
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
                    <div className="font-normal">{props.title}</div>
                  </div>
                  <Row className="justify-between items-center px-5">
                    <UserBadge
                      profile_url={user.profile_url ?? ''}
                      name={user.username}
                      author_type={UserType.Individual}
                    />
                    <TimeAgo timestamp={props.updated_at} />
                  </Row>
                  <Row className="justify-between px-5"></Row>
                  <FeedContents contents={props.contents} url={props.url} />
                </Col>
              ))}
          </Col>
        ) : (
          <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
            No drafts available
          </div>
        )}
      </div>

      <div className="w-80 pl-4 max-tablet:!hidden">
        <CreatePostButton />
      </div>
    </div>
  );
}
