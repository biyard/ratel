import { useMutation, InfiniteData } from '@tanstack/react-query';
import { getQueryClient } from '@/providers/getQueryClient';
import { feedKeys } from '@/constants';
import { Feed, FeedListResponse, FeedStatus } from '@/lib/api/models/feeds';
import { showErrorToast } from '@/lib/toast';
import { apiFetch } from '@/lib/api/apiFetch';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { writeCommentRequest } from '@/lib/api/models/feeds/comment';
import { UpdatePostRequest } from '@/lib/api/models/feeds/update-post';
import {
  createPost,
  CreatePostResponse,
  publishPost,
} from '@/lib/api/ratel/posts.v3';

export async function createDraft(): Promise<CreatePostResponse> {
  return createPost();
}

// TODO: Update to use v3 post API with string IDs
export async function updatePost(
  post_id: number | string,
  req: Partial<UpdatePostRequest>,
): Promise<Feed> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.updateDraft(post_id)}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(req),
    },
  );
  if (!res.data) throw new Error('Failed to update draft');
  return res.data;
}

export async function publishDraft(id: number): Promise<Feed> {
  const res = await apiFetch<Feed>(
    `${config.api_url}${ratelApi.feeds.publishDraft(id)}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ publish: {} }),
    },
  );
  if (!res.data) throw new Error('Failed to publish draft');
  return res.data;
}

export async function createComment(
  userId: number,
  parentId: number,
  content: string,
) {
  await apiFetch(`${config.api_url}${ratelApi.feeds.comment()}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(writeCommentRequest(content, userId, parentId)),
  });
}

// TODO: Update to use v3 feed query keys without userId parameter
export function useDraftMutations() {
  const queryClient = getQueryClient();

  const createMutation = useMutation({
    mutationFn: () => createDraft(),
    onSuccess: (newDraft) => {
      queryClient.setQueryData(feedKeys.detail(newDraft.post_pk), newDraft);

      // const listQueryKey = feedKeys.list({
      //   userId: listId,
      //   status: FeedStatus.Draft,
      // });

      // queryClient.setQueriesData<InfiniteData<FeedListResponse>>(
      //   { queryKey: listQueryKey },
      //   (oldData) => {
      //     if (!oldData) {
      //       return {
      //         pageParams: [1],
      //         pages: [{ posts: [newDraft], is_ended: false }],
      //       };
      //     }
      //     const newPages = [...oldData.pages];
      //     if (newPages.length === 0) {
      //       newPages.push({ posts: [newDraft], is_ended: false });
      //     } else {
      //       const first = newPages[0];
      //       newPages[0] = { ...first, posts: [newDraft, ...first.posts] };
      //     }
      //     return { ...oldData, pages: newPages };
      //   },
      // );
    },
    onError: (error: Error) => {
      throw new Error(error.message || 'Failed to create draft');
    },
  });

  const updateMutation = useMutation({
    mutationFn: ({
      postId,
      req,
      teamId = undefined,
    }: {
      postId: string;
      req: Partial<UpdatePostRequest>;
      teamId?: number;
    }) => {
      req.team_id = teamId;
      return updatePost(postId, req);
    },

    onMutate: async ({ postId, req }) => {
      const detailQueryKey = feedKeys.detail(postId);
      // TODO: Update to use v3 feed query keys without userId
      const listQueryKey = feedKeys.list({
        status: FeedStatus.Draft,
      });

      await queryClient.cancelQueries({ queryKey: detailQueryKey });
      await queryClient.cancelQueries({ queryKey: listQueryKey });

      const previousFeedDetail = queryClient.getQueryData<Feed>(detailQueryKey);
      const previousFeedLists = queryClient.getQueriesData<
        InfiniteData<FeedListResponse>
      >({ queryKey: listQueryKey });

      queryClient.setQueryData<Feed>(detailQueryKey, (old) =>
        old
          ? {
              ...old,
              html_contents: req.html_contents ?? old.html_contents,
              title: req.title ?? old.title,
              url:
                req.url !== undefined
                  ? req.url === null
                    ? old.url
                    : req.url
                  : old.url,
              url_type: req.url_type ?? old.url_type,
              feed_type: req.feed_type ?? old.feed_type,
              artwork_metadata: req.artwork_metadata ?? old.artwork_metadata,
            }
          : undefined,
      );

      queryClient.setQueriesData<InfiniteData<FeedListResponse>>(
        { queryKey: listQueryKey },
        (oldData) => {
          if (!oldData) return oldData;
          const newPages = oldData.pages.map((page) => ({
            ...page,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            posts: page.posts.map((post: any) =>
              post.id === postId
                ? {
                    ...post,
                    html_contents: req.html_contents ?? post.html_contents,
                    title: req.title ?? post.title,
                    url:
                      req.url !== undefined
                        ? req.url === null
                          ? post.url
                          : req.url
                        : post.url,
                    url_type: req.url_type ?? post.url_type,
                    feed_type: req.feed_type ?? post.feed_type,
                    artwork_metadata:
                      req.artwork_metadata ?? post.artwork_metadata,
                  }
                : post,
            ),
          }));
          return { ...oldData, pages: newPages };
        },
      );

      return { previousFeedDetail, previousFeedLists };
    },
    onError: (error, variables, context) => {
      if (context?.previousFeedDetail) {
        queryClient.setQueryData(
          feedKeys.detail(variables.postId),
          context.previousFeedDetail,
        );
      }
      if (context?.previousFeedLists) {
        context.previousFeedLists.forEach(([key, data]) => {
          queryClient.setQueryData(key, data);
        });
      }
      showErrorToast(error.message || 'Failed to update draft');
    },
    onSettled: (_data, _error, variables) => {
      queryClient.invalidateQueries({
        queryKey: feedKeys.detail(variables.postId),
      });
      queryClient.invalidateQueries({ queryKey: feedKeys.lists() });
    },
  });

  const publishMutation = useMutation({
    mutationFn: async ({
      draftId,
      title,
      content,
    }: {
      draftId: string;
      title: string;
      content: string;
    }) => {
      await publishPost(draftId, title, content);
      return draftId;
    },
    onSuccess: (draftId: string) => {
      queryClient.invalidateQueries({ queryKey: feedKeys.lists() });
      queryClient.invalidateQueries({ queryKey: feedKeys.detail(draftId) });
    },
    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to publish draft');
    },
  });

  const commentMutation = useMutation({
    mutationFn: ({
      userId,
      parentId,
      content,
    }: {
      userId: number;
      parentId: number;
      postId: number;
      content: string;
    }) => createComment(userId, parentId, content),
    onSuccess: (_, { postId }) => {
      // TODO: Update to use v3 feed API with string IDs
      queryClient.invalidateQueries({
        queryKey: feedKeys.detail(String(postId)),
      });
    },
    onError: (error: Error) => {
      showErrorToast(error.message || 'Failed to create comment');
    },
  });

  return {
    createDraft: createMutation,
    updateDraft: updateMutation,
    publishDraft: publishMutation,
    createComment: commentMutation,
  };
}
