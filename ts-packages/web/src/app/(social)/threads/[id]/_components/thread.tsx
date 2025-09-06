'use client';

import { useFeedByID } from '@/app/(social)/_hooks/feed';
import BlackBox from '@/app/(social)/_components/black-box';
import Image from 'next/image';
import { File } from '@/components/file';
import LexicalHtmlViewer from '../../../../../components/lexical/lexical-html-viewer';
import { useTranslations } from 'next-intl';

export default function Thread({ post_id }: { post_id: number }) {
  const t = useTranslations('Threads');
  const { data: post } = useFeedByID(post_id);

  return (
    <div className="flex flex-col w-full gap-2.5">
      <BlackBox>
        <div className="flex flex-col gap-5 w-full">
          <LexicalHtmlViewer htmlString={post?.html_contents || ''} />
          {post?.url && (
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
