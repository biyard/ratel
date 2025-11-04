import { PostDetailResponse } from '@/features/posts/dto/post-detail-response';
import Card from '@/components/card';
import Post, { PostType } from '@/features/posts/types/post';
import { PostEditor } from '@/features/posts/components/post-editor';
import { ArtworkTrait } from '@/features/posts/types/post-artwork';

export type ThreadPostProps = {
  feed: PostDetailResponse;
};

export default function ThreadPost({ feed }: ThreadPostProps) {
  console.log('ThreadPost feed', feed);
  return (
    <div className="flex flex-col w-full gap-2.5">
      <Card variant="secondary">
        {feed?.post?.post_type === PostType.Artwork ? (
          <ArtworkPost
            post={feed.post}
            artworkMetadata={feed.artwork_metadata}
          />
        ) : (
          <GeneralPost post={feed.post} />
        )}
      </Card>
    </div>
  );
}

export function ArtworkPost({
  post,
  artworkMetadata,
}: {
  post: Post;
  artworkMetadata?: ArtworkTrait[];
}) {
  const imageUrl = post.urls && post.urls.length > 0 ? post.urls[0] : null;
  const backgroundColor =
    artworkMetadata?.find((trait) => trait.trait_type === 'background_color')
      ?.value || '#ffffff';

  return (
    <div className="flex flex-col md:flex-row w-full min-h-[600px]">
      {/* Left side - Image */}
      <div className="flex justify-center items-center flex-1">
        <div
          className="flex flex-col justify-center p-5"
          style={{ backgroundColor }}
        >
          {imageUrl ? (
            <img
              src={imageUrl}
              alt={post.title}
              className="object-contain max-w-full max-h-[800px]"
            />
          ) : (
            <div className="text-text-secondary">No image available</div>
          )}
        </div>
      </div>

      {/* Right side - Traits and Description */}
      <div className="flex flex-col gap-6 flex-1 p-8 bg-card">
        <div className="flex flex-col gap-1">
          <p className="text-sm text-text-secondary">Artwork Name</p>
          <h1 className="text-2xl font-bold text-text-primary">{post.title}</h1>
        </div>
        {/* Traits */}
        {artworkMetadata && artworkMetadata.length > 0 && (
          <div className="flex flex-col gap-4">
            <h2 className="text-md font-semibold text-text-primary">
              Artwork Metadata
            </h2>
            <div className="flex flex-col gap-3">
              {artworkMetadata
                .filter((trait) => trait.trait_type !== 'background_color')
                .map((trait, index) => (
                  <div
                    key={index}
                    className="flex justify-between items-start p-3 rounded-lg bg-background"
                  >
                    <span className="text-sm font-medium text-text-secondary capitalize">
                      {trait.trait_type.replace(/_/g, ' ')}
                    </span>
                    <span className="text-xs text-text-secondary font-semibold">
                      {String(trait.value)}
                    </span>
                  </div>
                ))}
            </div>
          </div>
        )}

        {/* Description */}
        {post.html_contents && (
          <div className="flex flex-col gap-2">
            <h2 className="text-lg font-semibold text-text-primary">
              Description
            </h2>
            <div className="text-text-primary">
              <PostEditor
                content={post.html_contents}
                editable={false}
                showToolbar={false}
                url={null}
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

function GeneralPost({ post }: { post: Post }) {
  // const url = post.urls && post.urls.length > 0 ? post.urls[0] : null;

  return (
    <div className="flex flex-col gap-5 w-full">
      <PostEditor
        content={post.html_contents}
        editable={false}
        showToolbar={false}
        url={post.urls.length > 0 ? post.urls[0] : null}
      />
    </div>
  );
}
