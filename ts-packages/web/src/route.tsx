import { SpaceType } from './features/spaces/types/space-type';
import { RelationType } from './types/relation-type';

export const route = {
  home: () => '/',
  myProfile: () => '/my-profile',
  explore: () => '/explore',
  settings: () => '/settings',
  myPosts: () => '/my-posts',
  createPost: (postPk?: string) =>
    postPk ? `/posts/new?post-pk=${encodeURIComponent(postPk)}` : '/posts/new',
  drafts: () => '/drafts',
  draftEdit: (postPk: string) => `/drafts/${encodeURIComponent(postPk)}/edit`,
  teams: () => '/teams',
  groups: () => '/groups',

  login: () => '/login',
  signup: () => '/signup',
  connect: () => `/connect`,

  myNetwork: () => '/my-network',
  myFollower: (type: RelationType) => `/my-follower?type=${type}`,
  messages: () => '/messages',
  notifications: () => '/notifications',
  teamByUsername: (username: string) => `/teams/${username}/home`,
  teamGroups: (username: string) => `/teams/${username}/groups`,
  teamMembers: (username: string) => `/teams/${username}/members`,
  teamSettings: (username: string) => `/teams/${username}/settings`,
  teamDrafts: (username: string) => `/teams/${username}/drafts`,
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
  spaceByType: (spaceType: SpaceType, spaceId: number | string) => {
    switch (spaceType) {
      default:
        return `/spaces/${encodeURIComponent(spaceId)}`;
    }
  },
  spacePolls: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls`,
  spacePanels: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/panels`,
  spacePollById: (spaceId: string, pollId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/${encodeURIComponent(pollId)}`,
  spaceAnalyzePolls: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/analyzes`,
  spaceAnalyzePollById: (spaceId: string, pollId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/polls/${encodeURIComponent(pollId)}/analyzes`,
  spaceFiles: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/files`,
  spaceDiscussions: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/discussions`,
  spaceRecommendations: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/recommendations`,
  spaceSprintLeagues: (spaceId: string) =>
    `/spaces/${encodeURIComponent(spaceId)}/sprint-leagues`,

  // Admin routes
  admin: () => '/admin',
  adminMemberships: () => '/admin/memberships',
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
