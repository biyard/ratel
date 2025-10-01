import { Feed, FeedStatus, FeedV2 } from './models/feeds';
import { FileType } from './models/file-type';
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
  QK_GET_DELIBERATION_SPACE_BY_SPACE_ID,
  QK_GET_FEED_BY_FEED_ID_V2,
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
import { GroupPermission } from './models/group';
import { DeliberationSpace } from './models/spaces/deliberation-spaces';

export function useDeliberationSpaceById(
  id: string,
): UseSuspenseQueryResult<DeliberationSpace> {
  const spacePk = 'DELIBERATION_SPACE%23' + id;
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
    queryFn: () => get(ratelApi.spaces.getDeliberationSpaceBySpaceId(spacePk)),
    refetchOnWindowFocus: false,
  });

  return query;
}

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

export function usePostByIdV2(id: string): UseSuspenseQueryResult<FeedV2> {
  const feedPk = 'FEED%23' + id;
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_FEED_BY_FEED_ID_V2, feedPk],
    queryFn: () => get(ratelApi.feeds.getFeedById(feedPk)),
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
  permissions: {
    getPermissions: (teamId: number, permission: GroupPermission) =>
      `/v2/permissions?team_id=${teamId}&permission=${permission}`,
  },
  users: {
    login: () => '/v1/users?action=login',

    logout: () => '/v2/users/logout',
    loginWithPassword: (email: string, password: string) =>
      `/v1/users?action=login-by-password&email=${encodeURIComponent(email)}&password=${password}`,
    loginWithTelegram: (raw: string) =>
      `/v1/users?action=login-by-telegram&telegram_raw=${raw}`,
    // getUserInfo: () => '/v1/users?action=user-info',
    getUserInfo: () => '/v3/me',
    getUserByEmail: (email: string) => `/v3/users?email=${email}`,
    getUserByUsername: (username: string) => `/v3/users?username=${username}`,
    getUserByPhoneNumber: (phoneNumber: string) =>
      `/v3/users?phone-number=${phoneNumber}`,

    signup: () => '/v3/auth/signup',
    editProfile: (user_id: number) => `/v1/users/${user_id}`,
    updateEvmAddress: () => '/v1/users',

    updateTelegramId: () => '/v2/users/telegram',

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
    deleteTeam: () => '/v2/teams',
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

    //v3
    deliberation_response_answer: (spacePk: string) =>
      `/v3/spaces/deliberation/${spacePk}/responses`,
  },
  groups: {
    create_group: (team_id: number) => `/v1/teams/${team_id}/groups`,
    invite_member: (team_id: number, group_id: number) =>
      `/v1/teams/${team_id}/groups/${group_id}`,
    check_email: (team_id: number, group_id: number) =>
      `/v1/teams/${team_id}/groups/${group_id}`,
    delete_group: (team_id: number, group_id: number) =>
      `/v1/teams/${team_id}/groups/${group_id}`,
  },
  networks: {
    getNetworks: () => '/v1/network?param-type=read&action=find-one',
    follow: (user_id: number) => `/v1/my-networks/${user_id}`,
    unfollow: (user_id: number) => `/v1/my-networks/${user_id}`,
  },
  news: {
    getNewsDetails: (news_id: number) => `/v1/news/${news_id}`,
    getNews: (page: number, size: number) =>
      `/v1/news?param-type=query&page=${page}&size=${size}`,
  },
  themes: {
    changeTheme: () => '/v2/themes',
  },
  binances: {
    createSubscription: () => '/v2/binances/subscriptions',
    unsubscribe: () => '/v2/binances/unsubscribe',
  },
  feeds: {
    comment: () => '/v1/feeds',
    writePost: () => '/v1/feeds',
    createDraft: () => '/v1/feeds',
    updateDraft: (post_id: number) => `/v2/feeds/${post_id}`,
    editPost: (post_id: number) => `/v1/feeds/${post_id}`,
    publishDraft: (post_id: number) => `/v1/feeds/${post_id}`,
    removeDraft: (post_id: number) => `/v1/feeds/${post_id}?action=delete`,
    likePost: (post_id: number) => `/v1/feeds/${post_id}`,
    repost: () => '/v1/feeds',
    unrepost: (post_id: number) => `/v1/feeds/${post_id}?action=unrepost`,

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

    getFeed: (post_id: number) => `/v2/feeds/${post_id}`,
    getFeeds: (
      page: number,
      size: number,
      user_id?: number,
      status?: FeedStatus,
    ) => {
      let url = `/v2/feeds?page=${page}&size=${size}`;
      if (user_id) {
        url += `&user_id=${user_id}`;
      }
      if (status) {
        url += `&status=${status}`;
      }
      return url;
    },

    //V3
    getFeedById: (feed_pk: string) => `/v3/posts/${feed_pk}`,
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
    deleteSpaceById: (id: number) => `/v1/spaces/${id}?action=delete`,
    // v2 delete space endpoint requiring confirmation payload
    deleteSpaceV2: (id: number) => `/v2/spaces/${id}/delete`,
    getSpaceRedeemCodes: (space_id: number) =>
      `/v1/spaces/${space_id}/redeem-codes`,
    getUserBadge: (space_id: number, page: number, size: number) =>
      `/v1/spaces/${space_id}/badges?param-type=query&bookmark=${page}&size=${size}`,
    claimBadge: (space_id: number) => `/v1/spaces/${space_id}/badges`,

    // v3 api
    getDeliberationSpaceBySpaceId: (space_pk: string) =>
      `/v3/spaces/deliberation/${space_pk}`,
    updateDeliberationSpaceBySpaceId: (space_pk: string) =>
      `/v3/spaces/deliberation/${space_pk}`,
    postingDeliberationSpaceBySpaceId: (space_pk: string) =>
      `/v3/spaces/deliberation/${space_pk}/posting`,
    deleteDeliberationSpaceBySpaceId: (space_pk: string) =>
      `/v3/spaces/deliberation/${space_pk}/delete`,
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
    createSprintLeague: (space_id: number) =>
      `/v1/spaces/${space_id}/sprint-leagues`,
    voteSprintLeague: (space_id: number, sprint_league_id: number) =>
      `/v1/spaces/${space_id}/sprint-leagues/${sprint_league_id}`,
    updateSprintLeaguePlayer: (
      spaceId: number,
      sprintLeagueId: number,
      playerId: number,
    ) => {
      return `/v1/spaces/${spaceId}/sprint-leagues/${sprintLeagueId}/players/${playerId}`;
    },
  },
  notifications: {
    getNotifications: (page: number, size: number, filterType?: string) => {
      const params = new URLSearchParams({
        'param-type': 'query',
        bookmark: page.toString(),
        size: size.toString(),
      });
      if (filterType && filterType !== 'all') {
        params.append('type', filterType);
      }
      return `/v1/notifications?${params.toString()}`;
    },
    dismiss: (notificationId: number) => `/v1/notifications/${notificationId}`,
    markAsRead: (notificationId: number) =>
      `/v1/notifications/${notificationId}`,
    markAllAsRead: () => `/v2/notifications/mark-all-read`,
  },
  dagit: {
    getDagitBySpaceId: (spaceId: number) => `/v2/dagits/${spaceId}`,
    addOracle: (spaceId: number) => `/v2/dagits/${spaceId}/oracles`,
    createArtwork: (spaceId: number) => `/v2/dagits/${spaceId}/artworks`,
    getArtworkById: (artworkId: number) => `/v2/artworks/${artworkId}`,
    getArtworkCertificate: (artworkId: number) =>
      `/v2/artworks/${artworkId}/certificate`,

    startConsensus: (spaceId: number) => `/v2/dagits/${spaceId}/consensus`,
    voteConsensus: (spaceId: number, artworkId: number) =>
      `/v2/dagits/${spaceId}/artworks/${artworkId}/vote`,
  },
  telegram: {
    verifyTelegramRaw: () => `/v2/telegram`,
  },
  home: {
    getHomeData: (feedLimit?: number, newsLimit?: number) => {
      const params = new URLSearchParams();
      if (feedLimit) params.append('feed_limit', feedLimit.toString());
      if (newsLimit) params.append('news_limit', newsLimit.toString());
      const queryString = params.toString();
      return `/wg/home${queryString ? `?${queryString}` : ''}`;
    },
  },
};
