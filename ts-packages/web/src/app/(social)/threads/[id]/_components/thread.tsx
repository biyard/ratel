import { ArtworkPost } from '@/app/(social)/_components/post-editor';
import LexicalHtmlViewer from '@/components/lexical/lexical-html-viewer';
import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';
import Card from '@/components/card';
import Post from '@/features/posts/types/post';

export type ThreadPostProps = {
  feed: PostDetailResponse;
};

export default function ThreadPost({ feed }: ThreadPostProps) {
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Card variant="secondary">
        {feed?.post?.post_type === 3 ? (
          <Artwork post={feed.post} metadata={feed.artwork_metadata} />
        ) : (
          <GeneralPost post={feed.post} />
        )}
      </Card>
    </div>
  );
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function Artwork({ post, metadata }: { post: Post; metadata?: any }) {
  const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="w-full h-full">
      <ArtworkPost
        editMode={false}
        title={post.title || ''}
        content={post.html_contents || ''}
        image={url}
        traits={metadata.traits || []}
      />
    </div>
  );
}
function GeneralPost({ post }: { post: Post }) {
  const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="flex flex-col gap-5 w-full">
      <LexicalHtmlViewer htmlString={post.html_contents || ''} />
      {url && (
        <div className="relative h-72 w-full rounded-[10px]">
          <img
            className="object-contain"
            src={url}
            alt={post.title || 'Post Image'}
          />
        </div>
      )}
    </div>
  );
}
