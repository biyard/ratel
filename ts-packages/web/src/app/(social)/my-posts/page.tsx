'use client';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import React from 'react';
import { usePostByUserId } from '../_hooks/use-posts';
import { Col } from '@/components/ui/col';
import FeedCard from '@/components/feed-card';
import { logger } from '@/lib/logger';
import { Post } from '../page.client';
import CreatePostButton from '../_components/create-post-button';
import { checkString } from '@/lib/string-filter-utils';

export default function MyPostsPage() {
  const { data: user } = useSuspenseUserInfo();
  const user_id = user.id || 0;
  const posts = usePostByUserId(user_id, 1, 20);
  const data = posts.data;
  logger.debug('query response of posts', data);

  const feeds: Post[] = data.items.map((item) => ({
    id: item.id,
    industry: item.industry[0].name,
    title: item.title!,
    contents: item.html_contents,
    url: item.url,
    author_id: Number(item.author[0].id),
    author_profile_url: item.author[0].profile_url!,
    author_name: item.author[0].nickname,
    author_type: item.author[0].user_type,

    likes: item.likes,
    is_liked: item.is_liked,
    comments: item.comments,
    rewards: item.rewards,
    shares: item.shares,
    created_at: item.created_at,
    onboard: item.onboard || false,
  }));

  return (
    <div className="flex-1 flex relative">
      <div className="flex-1 flex max-mobile:px-[10px]">
        {feeds.length != 0 ? (
          <Col className="flex-1 border-gray-800">
            {feeds
              .filter(
                (d) =>
                  !(
                    checkString(d.title) ||
                    checkString(d.contents) ||
                    checkString(d.author_name)
                  ),
              )
              .map((props) => (
                <FeedCard
                  key={`feed-${props.id}`}
                  user_id={user_id ?? 0}
                  refetch={() => posts.refetch()}
                  {...props}
                />
              ))}
          </Col>
        ) : (
          <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
            Feeds data is empty
          </div>
        )}
      </div>

      <div className="w-80 pl-4 max-tablet:!hidden">
        <CreatePostButton />
      </div>
    </div>
  );
}
