import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { PollSpaceResponse } from '@/lib/api/ratel/poll.spaces.v3';
import usePollSpace from '@/features/poll-space/hooks/use-poll-space';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';

// Base class with common implementations
interface PollSpaceController {
  post: PostDetailResponse;
  space: PollSpaceResponse;
  onSave: (title: string, html_content: string) => Promise<void>;
}

export function usePollSpaceController(spacePk: string): PollSpaceController {
  const { data: space } = usePollSpace(spacePk);
  const { data: post } = useFeedById(space.post_pk);

  const onSave = async (title: string, html_content: string) => {
    // Implement save logic here
    console.log('Save changes', { title, html_content });
  };
  return {
    post,
    space,
    onSave,
  };
}
