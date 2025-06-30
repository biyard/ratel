import { SSRHydration } from '@/lib/query-utils';
import { getQueryClient } from '@/providers/getQueryClient';
import { Metadata } from 'next';
import striptags from 'striptags';
import { cache, Suspense } from 'react';
import { notFound } from 'next/navigation';
import { logger } from '@/lib/logger';
import News from './_components/news';
import { requestNewsByID, setInitialNewsByID } from '../../_hooks/news';
import NewsHeader from './_components/header';

interface Props {
  params: Promise<{ id: string }>;
}

const getCachedNewsByID = cache(async (id: number) => {
  return requestNewsByID(id);
});

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { id } = await params;
  const news_id = parseInt(id, 10);

  let title = 'Ratel News';
  let description = 'Ratel News';
  let image = '';
  try {
    const { data: news } = await getCachedNewsByID(news_id);
    if (news) {
      title = news.title || title;
      description = striptags(news.html_content);
      image = news.title || '';
    }
  } catch (error) {
    logger.error(`Failed to generate metadata for news ${news_id}:`, error);
  }

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images: [
        {
          url: image,
        },
      ],
    },
  };
}

export default async function Page({ params }: Props) {
  const { id } = await params;
  const news_id = parseInt(id, 10);
  const queryClient = getQueryClient();
  const { data } = await getCachedNewsByID(news_id);
  if (!data) {
    notFound();
  }

  setInitialNewsByID(queryClient, news_id, data);

  return (
    <SSRHydration queryClient={queryClient}>
      <Suspense fallback={<div>Loading...</div>}>
        <div className="flex flex-col gap-6 w-full">
          <NewsHeader news_id={news_id} />
          <News news_id={news_id} />
        </div>
      </Suspense>
    </SSRHydration>
  );
}
