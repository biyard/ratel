import Post from '../types/post';
import PostComment from '../types/post-comment';

export interface PostDetailResponse {
  post: Post;
  is_liked: boolean;
  comments: PostComment[];
  // FIXME: Define the type properly
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  artwork_metadata?: any;
  permissions: bigint;
}
