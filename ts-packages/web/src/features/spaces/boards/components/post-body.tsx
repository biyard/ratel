import Card from '@/components/card';
import { PostEditor } from '@/features/posts/components/post-editor';
import { SpacePostResponse } from '../types/space-post-response';

export type PostBodyProps = {
  post: SpacePostResponse;
};

export default function PostBody({ post }: PostBodyProps) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Card variant="secondary">
        <GeneralPost post={post} />
      </Card>
    </div>
  );
}

function GeneralPost({ post }: { post: SpacePostResponse }) {
  // const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="flex flex-col gap-5 w-full">
      <PostEditor
        content={post?.html_contents}
        editable={false}
        showToolbar={false}
        url={post?.urls.length > 0 ? post?.urls[0] : null}
        files={post?.files.length > 0 ? post?.files : []}
      />
    </div>
  );
}
