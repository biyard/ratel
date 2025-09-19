'use client';
import { LoadingIndicator } from '@/app/loading';
import { Col } from '@/components/ui/col';
import { ratelApi } from '@/lib/api/ratel_api';
import { useSuspenseQuery } from '@tanstack/react-query';
import { useTranslations } from 'next-intl';
import {
  ErrorBoundary,
  ErrorComponent,
} from 'next/dist/client/components/error-boundary';
import { useRouter } from 'next/navigation';
import React, { Suspense } from 'react';
import DisableBorderCard from './disable-border-card';
import { useApiCall } from '@/lib/api/use-send';

export interface NewsItem {
  id: number;
  title: string;
  html_content: string;
  created_at: number;
}

const Error: ErrorComponent = ({ error }) => {
  console.error('Error occurred:', error);
  return <div></div>;
};
export default function Wrapper() {
  return (
    <ErrorBoundary errorComponent={Error}>
      <Suspense fallback={<LoadingIndicator />}>
        <News />
      </Suspense>
    </ErrorBoundary>
  );
}
function News() {
  const t = useTranslations('Home');
  const router = useRouter();
  const { get } = useApiCall();

  const { data: news } = useSuspenseQuery<NewsItem[]>({
    queryKey: ['news', 3],
    queryFn: async () => {
      // Backend returns Vec<NewsSummary>; useApiCall.get returns parsed JSON.
      return ((await get(ratelApi.news.list(3))) ?? []) as NewsItem[];
    },
    refetchOnWindowFocus: false,
  });

  const handleNewsNavigation = (id: number) => {
    router.push(`/news/${id}`);
  };
  return (
    <DisableBorderCard>
      <Col className="w-full rounded-[10px]">
        <h3 className="text-[15px]/[20px] tracking-[0.5px] font-bold text-text-primary">
          {t('latest_news')}
        </h3>
        <Col className="gap-3.75">
          {news.map((item) => (
            <Col
              onClick={() => handleNewsNavigation(item.id)}
              key={`news-${item.id}`}
              className="py-2.5 cursor-pointer"
            >
              <h4 className="text-base/[25px] tracking-[0.5px] align-middle font-medium text-text-primary">
                {item.title}
              </h4>
              <div
                className="text-sm/[20px] align-middle font-light line-clamp-2 whitespace-normal text-text-primary"
                dangerouslySetInnerHTML={{
                  __html: item.html_content || '',
                }}
              ></div>
            </Col>
          ))}
        </Col>
      </Col>
    </DisableBorderCard>
  );
}
