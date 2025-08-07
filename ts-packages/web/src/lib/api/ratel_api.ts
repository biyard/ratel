import { Feed, FeedStatus } from './models/feeds';
import { FileType } from './models/file-type';
import { gql } from '@apollo/client';
import { Space } from './models/spaces';
import {
  QuizAttempt,
  QuizAttemptsResponse,
  NoticeQuizAnswer,
} from './models/notice';
import {
  QK_GET_FEED_BY_FEED_ID,
  QK_GET_NETWORK,
  QK_GET_PROMOTION,
  QK_GET_REDEEM_CODE,
  QK_GET_SPACE_BY_SPACE_ID,
  QK_LATEST_QUIZ_ATTEMPT,
  QK_QUIZ_ATTEMPTS,
  QK_QUIZ_ANSWERS,
} from '@/constants';
import {
  useSuspenseQuery,
  UseSuspenseQueryResult,
  useQuery,
  UseQueryResult,
} from '@tanstack/react-query';
import { useApiCall } from './use-send';
import { RedeemCode } from './models/redeem-code';
import { NetworkData } from './models/network';
import { Promotion } from './models/promotion';

export function useSpaceById(id: number): UseSuspenseQueryResult<Space> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_SPACE_BY_SPACE_ID, id],
    queryFn: () => get(ratelApi.spaces.getSpaceBySpaceId(id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useRedeemCode(
  meta_id: number,
): UseSuspenseQueryResult<RedeemCode> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_REDEEM_CODE, meta_id],
    queryFn: () => get(ratelApi.spaces.getSpaceRedeemCodes(meta_id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useFeedById(id: number): UseSuspenseQueryResult<Feed> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_FEED_BY_FEED_ID, id],
    queryFn: () => get(ratelApi.feeds.getFeedsByFeedId(id)),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useNetwork(): UseSuspenseQueryResult<NetworkData> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_NETWORK],
    queryFn: () => get(ratelApi.networks.getNetworks()),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function usePromotion(): UseSuspenseQueryResult<Promotion> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_PROMOTION],
    queryFn: () => get(ratelApi.promotions.get_promotions()),
    refetchOnWindowFocus: false,
  });

  return query;
}

export function useQuizAttempts(
  spaceId: number,
  enabled = true,
): UseQueryResult<QuizAttemptsResponse> {
  const { get } = useApiCall();

  return useQuery({
    queryKey: [QK_QUIZ_ATTEMPTS, spaceId],
    queryFn: async () => {
      if (!spaceId) {
        throw new Error('Space ID is required');
      }

      const response: QuizAttemptsResponse = await get(
        ratelApi.notice_quiz.getQuizAttempts(spaceId),
      );
      return response;
    },
    enabled: enabled && spaceId > 0,
  });
}

export function useLatestQuizAttempt(
  spaceId: number,
): UseQueryResult<QuizAttempt | null> {
  const { get } = useApiCall();

  const query = useQuery({
    queryKey: [QK_LATEST_QUIZ_ATTEMPT, spaceId],
    queryFn: async () => {
      const response: QuizAttemptsResponse = await get(
        ratelApi.notice_quiz.getLatestQuizAttempt(spaceId),
      );
      // Return the latest attempt (first item since it's ordered by created_at desc)
      return response.items.length > 0 ? response.items[0] : null;
    },
    refetchOnWindowFocus: true,
    refetchOnMount: true,
    refetchOnReconnect: true,
    staleTime: 5 * 1000, // Consider data fresh for 5 seconds
    gcTime: 10 * 60 * 1000, // Keep in cache for 10 minutes
    enabled: spaceId > 0,
    retry: 3, // Retry failed requests up to 3 times
    retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000), // Exponential backoff
  });

  return query;
}

export function useQuizAnswers(
  spaceId: number,
  enabled = true,
): UseQueryResult<NoticeQuizAnswer> {
  const { get } = useApiCall();

  return useQuery({
    queryKey: [QK_QUIZ_ANSWERS, spaceId],
    queryFn: async () => {
      if (!spaceId) {
        throw new Error('Space ID is required');
      }

      const response: NoticeQuizAnswer = await get(
        ratelApi.notice_quiz.getQuizAnswers(spaceId),
      );
      return response;
    },
    enabled: enabled && spaceId > 0,
  });
}

export const proxy = {
  login: {
    loginWithTelegram: (telegram_raw: string) =>
      `/api/login?telegram-raw=${btoa(telegram_raw)}`,
  },
};
export const ratelApi = {
  users: {
    login: () => '/v1/users?action=login',
    logout: () => '/v2/users/logout',
    loginWithPassword: (email: string, password: string) =>
      `/v1/users?action=login-by-password&email=${encodeURIComponent(email)}&password=${password}`,
    loginWithTelegram: (raw: string) =>
      `/v1/users?action=login-by-telegram&telegram_raw=${raw}`,
    getTotalInfo: (page: number, size: number) =>
      `/v1/totals?param-type=query&bookmark=${page}&size=${size}`,
    getUserInfo: () => '/v1/users?action=user-info',
    getUserByEmail: (email: string) => `/v2/users?email=${email}`,
    getUserByUsername: (username: string) => `/v2/users?username=${username}`,
    getUserByPhoneNumber: (phoneNumber: string) =>
      `/v2/users?phone-number=${phoneNumber}`,

    signup: () => '/v1/users?action=signup',
    editProfile: (user_id: number) => `/v1/users/${user_id}`,
    updateEvmAddress: () => '/v1/users',

    updateTelegramId: () => '/v1/users',

    sendVerificationCode: () => '/v1/users/verifications',
  },
  assets: {
    getPresignedUrl: (file_type: FileType, total_count = 1) =>
      `/v1/assets?action=get-presigned-uris&file_type=${file_type}&total_count=${total_count}`,
    getMultipartPresignedUrl: (file_type: FileType, total_count = 1) =>
      `/v1/assets/multipart?action=get-presigned-uris&file_type=${file_type}&total_count=${total_count}`,
    createMultipartUpload: () => `/v1/assets/multipart/complete`,
  },
  teams: {
    createTeam: () => '/v1/teams',
    getTeamById: (team_id: number) => `/v1/teams/${team_id}`,
    getTeamByUsername: (username: string) =>
      `/v1/teams?param-type=read&action=get-by-username&username=${username}`,
  },
  subscription: {
    subscribe: () => '/v1/subscriptions?action=subscribe',
  },
  promotions: {
    get_promotions: () => '/v1/promotions?param-type=read&action=hot-promotion',
  },
  responses: {
    respond_answer: (spaceId: number) => `/v1/spaces/${spaceId}/responses`,
  },
  groups: {
    create_group: (team_id: number) => `/v1/teams/${team_id}/groups`,
    invite_member: (team_id: number, group_id: number) =>
      `/v1/teams/${team_id}/groups/${group_id}`,
    check_email: (team_id: number, group_id: number) =>
      `/v1/teams/${team_id}/groups/${group_id}`,
  },
  networks: {
    getNetworks: () => '/v1/network?param-type=read&action=find-one',
    follow: (user_id: number) => `/v1/my-networks/${user_id}`,
    unfollow: (user_id: number) => `/v1/my-networks/${user_id}`,
  },
  news: {
    getNewsDetails: (news_id: number) => `/v1/news/${news_id}`,
  },
  feeds: {
    comment: () => '/v1/feeds',
    writePost: () => '/v1/feeds',
    createDraft: () => '/v1/feeds',
    updateDraft: (post_id: number) => `/v1/feeds/${post_id}`,
    editPost: (post_id: number) => `/v1/feeds/${post_id}`,
    publishDraft: (post_id: number) => `/v1/feeds/${post_id}`,
    removeDraft: (post_id: number) => `/v1/feeds/${post_id}?action=delete`,
    likePost: (post_id: number) => `/v1/feeds/${post_id}`,
    getPostsByUserId: (
      user_id: number,
      page: number,
      size: number,
      status: FeedStatus,
    ) =>
      `/v1/feeds?param-type=query&action=posts-by-user-id&bookmark=${page}&size=${size}&user-id=${user_id}&status=${status}`,
    getFeedsByFeedId: (feed_id: number) => `/v1/feeds/${feed_id}`,
    getPosts: (page: number, size: number) =>
      `/v1/feeds?param-type=query&bookmark=${page}&size=${size}`,
  },
  redeems: {
    useRedeemCode: (redeem_id: number) => `/v1/redeems/${redeem_id}`,
  },
  discussions: {
    getDiscussionById: (spaceId: number, discussionId: number) =>
      `/v1/spaces/${spaceId}/discussions/${discussionId}`,
    actDiscussionById: (spaceId: number, discussionId: number) =>
      `/v1/spaces/${spaceId}/discussions/${discussionId}`,
  },
  meeting: {
    getMeetingById: (spaceId: number, discussionId: number) =>
      `/v1/spaces/${spaceId}/meeting/${discussionId}?param-type=read&action=find-one`,
  },
  spaces: {
    createSpace: () => '/v1/spaces',
    likeSpace: (id: number) => `/v1/spaces/${id}`,
    shareSpace: (id: number) => `/v1/spaces/${id}`,
    getSpaceBySpaceId: (id: number) => `/v1/spaces/${id}`,
    getSpaceRedeemCodes: (space_id: number) =>
      `/v1/spaces/${space_id}/redeem-codes`,
    getUserBadge: (space_id: number, page: number, size: number) =>
      `/v1/spaces/${space_id}/badges?param-type=query&bookmark=${page}&size=${size}`,
    claimBadge: (space_id: number) => `/v1/spaces/${space_id}/badges`,
  },
  notice_quiz: {
    submitQuizAnswers: (id: number) =>
      `/v1/spaces/${id}/notice-quiz-attempts/submit`,
    getQuizAttempts: (spaceId: number) =>
      `/v1/spaces/${spaceId}/notice-quiz-attempts`,
    getLatestQuizAttempt: (spaceId: number) =>
      `/v1/spaces/${spaceId}/notice-quiz-attempts`,
    getQuizAnswers: (spaceId: number) =>
      `/v1/spaces/${spaceId}/notice-quiz-answers`,
  },
  sprint_league: {
    voteSprintLeague: (space_id: number, sprint_league_id: number) =>
      `/v1/spaces/${space_id}/sprint-leagues/${sprint_league_id}`,
  },
  telegram: {
    subscribe: () => '/v2/telegram/subscribe',
  },
  graphql: {
    listNews: (size: number) => {
      return {
        query: gql`
          query ListNews($limit: Int!) {
            news(limit: $limit, order_by: { created_at: desc }) {
              id
              title
              html_content
              created_at
            }
          }
        `,
        variables: {
          limit: size,
        },
      };
    },
    listIndustries: () => {
      return {
        query: gql`
          query ListIndustries {
            industries {
              id
              name
            }
          }
        `,
      };
    },
    getUserByUsername: (username: string) => {
      return {
        query: gql`
          query GetUserByUsername($username: String!) {
            users(where: { username: { _eq: $username } }) {
              id
            }
          }
        `,
        variables: {
          username,
        },
      };
    },

    getUserByEmail: (email: string) => {
      return {
        query: gql`
          query GetUserByEmail($email: String!) {
            users(where: { email: { _eq: $email } }) {
              id
            }
          }
        `,
        variables: {
          email,
        },
      };
    },

    getTeamByTeamname: (teamname: string) => {
      return {
        query: gql`
          query GetTeamByTeamname($teamname: String!) {
            users(where: { username: { _eq: $teamname } }) {
              id
              html_contents
              email
              created_at
              nickname
              parent_id
              profile_url
              user_type
              username
            }
          }
        `,
        variables: {
          teamname,
        },
      };
    },
  },
};
