'use client';

import { useNewsByID } from '@/app/(social)/_hooks/news';
import Image from 'next/image';
import LexicalHtmlViewer from '../../../../../components/lexical/lexical-html-viewer';
import DisableBorderCard from '@/app/(social)/_components/disable-border-card';

export default function News({ news_id }: { news_id: number }) {
  const { data: news } = useNewsByID(news_id);

  return (
    <div className="flex flex-col w-full gap-2.5">
      <DisableBorderCard>
        <div className="flex flex-col gap-5">
          <LexicalHtmlViewer htmlString={news?.html_content || ''} />
          {news?.user_id && (
            <div className="relative w-full h-72 rounded-[10px] overflow-hidden">
              <Image
                fill
                className="object-cover"
                src={''}
                alt={news.title || 'Post Image'}
              />
            </div>
          )}
        </div>
      </DisableBorderCard>
    </div>
  );
}
