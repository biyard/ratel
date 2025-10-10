import { PollSpaceResponse } from '@/lib/api/ratel/poll.spaces.v3';
import { Post } from '@/lib/api/ratel/posts.v3';

class PollSpaceController {
  protected isLoading: boolean;
  post: Post;
  displayData: PollSpaceResponse | null;

  //   constructor(post: Post, space: PollSpaceResponse | null) {
  //     this.post = post;
  //     this.displayData = space;
  //     this.isLoading = !space;
  //   }
  constructor() {
    this.post = {} as Post;
    this.displayData = null;
    this.isLoading = true;
  }
}

export function usePollSpaceController(spacePk: string) {
  return new PollSpaceController();
}
