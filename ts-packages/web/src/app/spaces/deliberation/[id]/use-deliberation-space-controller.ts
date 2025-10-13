import useDeliberationSpace from '@/features/deliberation-space/hooks/use-deliberation-space';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { DeliberationSpaceResponse } from '@/lib/api/ratel/deliberation.spaces.v3';
import { PostDetailResponse } from '@/lib/api/ratel/posts.v3';

interface DeliberationSpaceController {
  post: PostDetailResponse;
  space: DeliberationSpaceResponse;
  onSave: (title: string, html_content: string) => Promise<void>;
}

export function useDeliberationSpaceController(
  spacePk: string,
): DeliberationSpaceController {
  const { data: space } = useDeliberationSpace(spacePk);
  const { data: post } = useFeedById(space.post_pk);

  console.log('space: ', space, 'post: ', post);

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
