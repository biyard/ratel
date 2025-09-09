'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import Image from 'next/image';
import { File } from '@/components/file';
import LexicalHtmlViewer from '../../../../../components/lexical/lexical-html-viewer';
import { useTranslations } from 'next-intl';
import { Feed, FeedType } from '@/lib/api/models/feeds';
import useFeedById from '@/hooks/feeds/use-feed-by-id';
import { ArtworkPost } from '@/app/(social)/_components/post-editor';
export default function Thread({ postId }: { postId: number }) {
  const t = useTranslations('Threads');

  const { data: post } = useFeedById(postId);

  return (
    <div className="flex flex-col w-full gap-2.5">
      <BlackBox>
        {post?.feed_type === FeedType.Artwork ? (
          <Artwork post={post} />
        ) : (
          <GeneralPost post={post} />
        )}
      </BlackBox>
      {post?.files && post.files.length > 0 && (
        <BlackBox>
          <div className="flex flex-col w-full gap-5">
            <div className="font-bold text-white text-[15px]/[20px]">
              {t('attached_files')}
            </div>

            <div className="grid grid-cols-2 max-tablet:grid-cols-1 gap-2.5">
              {post?.files.map((file, index) => (
                <File file={file} key={'file ' + index} />
              ))}
            </div>
          </div>
        </BlackBox>
      )}
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
        artistName={artworkMetadata.artist_name}
        backgroundColor={artworkMetadata.background_color}
        size={artworkMetadata.size}
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
          <Image
            fill
            className="object-contain"
            src={post.url}
            alt={post.title || 'Post Image'}
          />
        </div>
      )}
    </div>
  );
}
