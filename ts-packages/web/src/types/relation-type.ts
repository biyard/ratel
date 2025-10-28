export const RelationType = {
  FOLLOWER: 'followers',
  FOLLOWING: 'followings',
} as const;

export type RelationType = (typeof RelationType)[keyof typeof RelationType];
