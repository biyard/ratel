export interface FollowRequest {
  follow: object;
}

export interface UnFollowRequest {
  unfollow: object;
}

export function followRequest(): FollowRequest {
  return {
    follow: {},
  };
}

export function unfollowRequest(): UnFollowRequest {
  return {
    unfollow: {},
  };
}
