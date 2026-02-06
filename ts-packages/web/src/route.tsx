import { SpaceType } from './features/spaces/types/space-type';
import { RelationType } from './types/relation-type';

export const route = {
  // User Menus
  home: () => '/',
  myProfile: () => '/my-profile',
  explore: () => '/explore',
  settings: () => '/settings',
  credentials: () => '/credentials',
  rewards: () => '/rewards',
  myPosts: () => '/my-posts',
  mySpaces: () => '/my-spaces',

  createPost: (postPk?: string) =>
    postPk ? `/posts/new?post-pk=${encodeURIComponent(postPk)}` : '/posts/new',
  createArtwork: (postPk?: string) =>
    `/artworks/new?post-pk=${encodeURIComponent(postPk)}`,
  drafts: () => '/drafts',
  draftEdit: (postPk: string) => `/drafts/${encodeURIComponent(postPk)}/edit`,
  teams: () => '/teams',
  groups: () => '/groups',

  login: () => '/login',
  signup: () => '/signup',
  forgotPassword: () => '/forgot-password',
  resetPassword: () => '/reset-password',
  connect: () => `/connect`,

  myNetwork: () => '/my-network',
  membership: () => '/membership',
  myMembership: () => '/my-membership',
  myFollower: (type: RelationType) => `/my-follower?type=${type}`,
  messages: () => '/messages',
  notifications: () => '/notifications',
  teamByUsername: (username: string) => `/teams/${username}/home`,
  teamGroups: (username: string) => `/teams/${username}/groups`,
  teamMembers: (username: string) => `/teams/${username}/members`,
  teamSettings: (username: string) => `/teams/${username}/settings`,
  teamDrafts: (username: string) => `/teams/${username}/drafts`,
  teamDao: (username: string) => `/teams/${username}/dao`,
  teamRewards: (username: string) => `/teams/${username}/rewards`,
  teamDraftEdit: (username: string, postPk: string) =>
    `/teams/${username}/drafts/${encodeURIComponent(postPk)}/edit`,
  space: (spaceId: number | string) => `/spaces/${encodeURIComponent(spaceId)}`,

  commiteeSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}`,
  deliberationSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}/deliberation`,
  noticeSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}`,
  threadByFeedId: (feedId: number | string) => {
    return `/threads/${encodeURIComponent(feedId)}`;
  },
  discussionById: (spaceId: number, discussionId: number) =>
    `/spaces/${spaceId}/discussions/${discussionId}`,

  discussionByPk: (spacePk: string, discussionPk: string) =>
    `/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}`,

  spaceSetting: (spacePk: string) =>
    `/spaces/${encodeURIComponent(spacePk)}/settings`,
  spaceReward: (spacePk: string) =>
    `/spaces/${encodeURIComponent(spacePk)}/rewards`,
  spaceByType: (spaceType: SpaceType, spaceId: number | string) => {
    switch (spaceType) {
      default:
        return `/spaces/${encodeURIComponent(spaceId)}`;
    }
  },
  spaceBoards: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/boards`,
  spaceIncentiveSetting: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/incentive-setting`,
  spaceIncentive: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/incentive`,
  spaceCreatePost: (spaceId: string, postPk?: string) => {
    let to = `/spaces/${encodeURIComponent(spaceId)}/boards/create`;
    const params: string[] = [];

    if (postPk) {
      params.push(`post-pk=${encodeURIComponent(postPk)}`);
    }

    if (params.length > 0) {
      to += `?${params.join('&')}`;
    }

    return to;
  },
  spaceBoardPost: (spaceId: string, spacePostId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/boards/posts/${encodeURIComponent(spacePostId)}`,
  spacePolls: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls`,
  spacePanels: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/panels`,
  spaceHome: (spaceId: string) => `/spaces/${encodeURIComponent(spaceId)}`,
  spaceMembers: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/members`,
  spacePollById: (spaceId: string, pollId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/${encodeURIComponent(pollId)}`,
  spaceAnalyzePolls: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/analyzes`,
  spaceAnalyzePollById: (spaceId: string, pollId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/${encodeURIComponent(pollId)}/analyzes`,
  spaceFiles: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/files`,
  spacePdfViewer: (spaceId: string, fileId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/files/${encodeURIComponent(fileId)}`,
  spaceDiscussions: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/discussions`,
  spaceRecommendations: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/recommendations`,
  spaceSprintLeagues: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/sprint-leagues`,

  spaceNftPreview: (spacePk: string) =>
    `/spaces/${encodeURIComponent(spacePk)}/nfts`,
  spaceNftArtTwin: (spacePk: string) =>
    `/spaces/${encodeURIComponent(spacePk)}/nfts/art-twin`,
  spaceRequirements: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/requirements`,

  // Admin routes
  admin: () => '/admin',
  adminMemberships: () => '/admin/memberships',
  adminAttributeCodes: () => '/admin/attribute-codes',
  adminUsers: () => '/admin/users',
  adminRewards: () => '/admin/rewards',
  adminMigrations: () => '/admin/migrations',
  newPost: (postPk?: string, teamPk?: string) => {
    let to = '/posts/new';
    const params: string[] = [];

    if (teamPk) {
      params.push(`team-pk=${encodeURIComponent(teamPk)}`);
    }
    if (postPk) {
      params.push(`post-pk=${encodeURIComponent(postPk)}`);
    }

    if (params.length > 0) {
      to += `?${params.join('&')}`;
    }

    return to;
  },
};
