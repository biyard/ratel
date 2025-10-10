import LexicalHtmlViewer from '../../../../../components/lexical/lexical-html-viewer';
import { Feed } from '@/lib/api/models/feeds';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { ArtworkPost } from '@/app/(social)/_components/post-editor';
import DisableBorderCard from '@/app/(social)/_components/disable-border-card';

export default function Thread({ postId }: { postId: string }) {
  const { data: post } = useFeedById(postId);

  return (
    <div className="flex flex-col w-full gap-2.5">
      <DisableBorderCard>
        {post?.post?.post_type === 3 ? (
          /* 3 = Artwork in v3 */
          <Artwork post={post.post as unknown as Feed} />
        ) : (
          <GeneralPost post={post.post as unknown as Feed} />
        )}
      </DisableBorderCard>
      {/* TODO: Implement files support in v3 */}
      {/* Files section disabled until v3 API supports file attachments
      {post?.files && post.files.length > 0 && (
        <DisableBorderCard>
          <div className="flex flex-col w-full gap-5">
            <div className="font-bold text-text-primary text-[15px]/[20px]">
              {t('attached_files')}
            </div>

            <div className="grid grid-cols-2 max-tablet:grid-cols-1 gap-2.5">
              {post?.files.map((file, index) => (
                <File file={file} key={'file ' + index} />
              ))}
            </div>
          </div>
        </DisableBorderCard>
      )}
      */}
    </div>
  );
}

function Artwork({ post }: { post: Feed }) {
  const artworkMetadata = post.artwork_metadata;
  if (!artworkMetadata) {
    return null;
  }

  return (
    <div className="w-full h-full">
      <ArtworkPost
        editMode={false}
        title={post.title || ''}
        content={post.html_contents || ''}
        image={post.url || ''}
        traits={artworkMetadata.traits || []}
      />
    </div>
  );
}
function GeneralPost({ post }: { post: Feed }) {
  return (
    <div className="flex flex-col gap-5 w-full">
      <LexicalHtmlViewer htmlString={post.html_contents || ''} />
      {post.url && (
        <div className="relative h-72 w-full rounded-[10px]">
          <img
            className="object-contain"
            src={post.url}
            alt={post.title || 'Post Image'}
          />
        </div>
      )}
    </div>
  );
}
