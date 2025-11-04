import Post from '../types/post';
import PostComment from '../types/post-comment';
import { ArtworkTrait } from '../types/post-artwork';

export interface PostDetailResponse {
  post: Post;
  is_liked: boolean;
  comments: PostComment[];
  artwork_metadata?: ArtworkTrait[];
  permissions: bigint;
}
