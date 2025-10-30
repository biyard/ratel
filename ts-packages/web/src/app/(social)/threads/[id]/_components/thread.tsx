import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';
import Card from '@/components/card';
import Post, { PostType } from '@/features/posts/types/post';
import { TiptapEditor } from '@/components/text-editor';

export type ThreadPostProps = {
  feed: PostDetailResponse;
};

export default function ThreadPost({ feed }: ThreadPostProps) {
  console.log('ThreadPost feed', feed);
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Card variant="secondary">
        {feed?.post?.post_type === PostType.Artwork ? (
          <ArtworkPost post={feed.post} />
        ) : (
          <GeneralPost post={feed.post} />
        )}
      </Card>
    </div>
  );
}

function ArtworkPost({ post }: { post: Post }) {
  // const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="w-full h-full">
      <TiptapEditor
        content={post.html_contents}
        editable={false}
        showToolbar={false}
      />
    </div>
  );
}

function GeneralPost({ post }: { post: Post }) {
  // const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="flex flex-col gap-5 w-full">
      <TiptapEditor
        content={post.html_contents}
        editable={false}
        showToolbar={false}
      />
    </div>
  );
}
