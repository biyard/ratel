import { QK_GET_PROMOTION, QK_GET_REDEEM_CODE } from '@/constants';
import {
  useSuspenseQuery,
  type UseSuspenseQueryResult,
} from '@tanstack/react-query';
import { useApiCall } from './use-send';
import type { RedeemCode } from './models/redeem-code';
import type { Promotion } from './models/promotion';
import { FeedStatus } from '@/features/posts/types/post';

// export function useSpaceById(id: number): UseSuspenseQueryResult<Space> {
//   const { get } = useApiCall();

//   const query = useSuspenseQuery({
//     queryKey: [QK_GET_SPACE_BY_SPACE_ID, id],
//     queryFn: () => get(ratelApi.spaces.getSpaceBySpaceId(id)),
//     refetchOnWindowFocus: false,
//   });

//   return query;
// }

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

// export function useNetwork(): UseSuspenseQueryResult<NetworkData> {
//   const { get } = useApiCall();

//   const query = useSuspenseQuery({
//     queryKey: [QK_GET_NETWORK],
//     queryFn: () => get(ratelApi.networks.getNetworks()),
//     refetchOnWindowFocus: false,
//   });

//   return query;
// }

export function usePromotion(): UseSuspenseQueryResult<Promotion> {
  const { get } = useApiCall();

  const query = useSuspenseQuery({
    queryKey: [QK_GET_PROMOTION],
    queryFn: () => get(ratelApi.promotions.get_promotions()),
    refetchOnWindowFocus: false,
  });

  return query;
}

// export function useQuizAttempts(
//   spaceId: number,
//   enabled = true,
// ): UseQueryResult<QuizAttemptsResponse> {
//   const { get } = useApiCall();

//   return useQuery({
//     queryKey: [QK_QUIZ_ATTEMPTS, spaceId],
//     queryFn: async () => {
//       if (!spaceId) {
//         throw new Error('Space ID is required');
//       }

//       const response: QuizAttemptsResponse = await get(
//         ratelApi.notice_quiz.getQuizAttempts(spaceId),
//       );
//       return response;
//     },
//     enabled: enabled && spaceId > 0,
//   });
// }

// export function useLatestQuizAttempt(
//   spaceId: number,
// ): UseQueryResult<QuizAttempt | null> {
//   const { get } = useApiCall();

//   const query = useQuery({
//     queryKey: [QK_LATEST_QUIZ_ATTEMPT, spaceId],
//     queryFn: async () => {
//       const response: QuizAttemptsResponse = await get(
//         ratelApi.notice_quiz.getLatestQuizAttempt(spaceId),
//       );
//       // Return the latest attempt (first item since it's ordered by created_at desc)
//       return response.items.length > 0 ? response.items[0] : null;
//     },
//     refetchOnWindowFocus: true,
//     refetchOnMount: true,
//     refetchOnReconnect: true,
//     staleTime: 5 * 1000, // Consider data fresh for 5 seconds
//     gcTime: 10 * 60 * 1000, // Keep in cache for 10 minutes
//     enabled: spaceId > 0,
//     retry: 3, // Retry failed requests up to 3 times
//     retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000), // Exponential backoff
//   });

//   return query;
// }

// export function useQuizAnswers(
//   spaceId: number,
//   enabled = true,
// ): UseQueryResult<NoticeQuizAnswer> {
//   const { get } = useApiCall();

//   return useQuery({
//     queryKey: [QK_QUIZ_ANSWERS, spaceId],
//     queryFn: async () => {
//       if (!spaceId) {
//         throw new Error('Space ID is required');
//       }

//       const response: NoticeQuizAnswer = await get(
//         ratelApi.notice_quiz.getQuizAnswers(spaceId),
//       );
//       return response;
//     },
//     enabled: enabled && spaceId > 0,
//   });
// }

export const ratelApi = {
  // permissions: {
  //   // DEPRECATED: Use embedded permissions in v3 team detail instead
  //   _legacy_getPermissions: (teamPk: string, permission: GroupPermission) =>
  //     `/v3/teams/permissions?team_pk=${teamPk}&permission=${permission}`,
  // },
  users: {
    // getUserInfo: () => '/v3/me',
    // getUserByEmail: (email: string) =>
    //   `/v3/users?type=email&value=${encodeURIComponent(email)}`,
    // getUserByUsername: (username: string) =>
    //   `/v3/users?type=username&value=${username}`,
    // getUserByPhoneNumber: (phoneNumber: string) =>
    //   `/v3/users?type=phone-number&value=${phoneNumber}`,
    // signup: () => '/v3/auth/signup',
    // editProfile: (user_id: number) => `/v1/users/${user_id}`,
    // updateEvmAddress: () => '/v1/users',
    // updateTelegramId: () => '/v2/users/telegram',
    // sendVerificationCode: () => '/v1/users/verifications',
  },

  // assets: {
  //   getPresignedUrl: (file_type: FileExtension, total_count = 1) =>
  //     `/v3/assets?file_type=${file_type}&total_count=${total_count}`,
  //   getMultipartPresignedUrl: (file_type: FileExtension, total_count = 1) =>
  //     `/v3/assets/multiparts?file_type=${file_type}&total_count=${total_count}`,
  //   createMultipartUpload: () => `/v3/assets/multiparts/complete`,
  // },
  teams: {
    createTeam: () => '/v3/teams',
    deleteTeam: (teamPk: string) => `/v3/teams/${teamPk}`,
    getTeamByPk: (teamPk: string) => `/v3/teams/${teamPk}`,
    getTeamByUsername: (username: string) => `/v3/teams?username=${username}`,
    updateTeam: (teamPk: string) => `/v3/teams/${teamPk}`,
    getTeamMembers: (teamPk: string) => `/v3/teams/${teamPk}/members`,
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
    createGroup: (teamPk: string) => `/v3/teams/${teamPk}/groups`,
    updateGroup: (teamPk: string, groupSk: string) =>
      `/v3/teams/${teamPk}/groups/${groupSk}`,
    deleteGroup: (teamPk: string, groupSk: string) =>
      `/v3/teams/${teamPk}/groups/${groupSk}`,
    addMember: (teamPk: string, groupSk: string) =>
      `/v3/teams/${teamPk}/groups/${groupSk}/member`,
    removeMember: (teamPk: string, groupSk: string) =>
      `/v3/teams/${teamPk}/groups/${groupSk}/member`,
  },
  networks: {
    getNetworks: () => '/v3/networks/suggestions',
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
    // TODO: Update to use v3 feed API with string IDs
    updateDraft: (post_id: number | string) => `/v2/feeds/${post_id}`,
    editPost: (post_id: number | string) => `/v1/feeds/${post_id}`,
    publishDraft: (post_id: number | string) => `/v1/feeds/${post_id}`,
    removeDraft: (post_id: number | string) =>
      `/v1/feeds/${post_id}?action=delete`,
    likePost: (post_id: number | string) => `/v1/feeds/${post_id}`,
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
