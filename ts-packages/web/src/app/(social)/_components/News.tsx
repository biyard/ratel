'use client';
import { useTheme } from '@/app/_providers/ThemeProvider';
import { Col } from '@/components/ui/col';
import { ratelApi } from '@/lib/api/ratel_api';
import { useSuspenseQuery } from '@apollo/client';
import { useRouter } from 'next/navigation';
import React from 'react';

export interface NewsItem {
  id: number;
  title: string;
  html_content: string;
  created_at: number;
}

export default function News() {
  const { theme } = useTheme();
  const router = useRouter();
  const q = ratelApi.graphql.listNews(3);
  const {
    data: { news },
  }: { data: { news: NewsItem[] } } = useSuspenseQuery(q.query, {
    variables: q.variables,
  });

  const handleNewsNavigation = (id: number) => {
    router.push(`/news/${id}`);
  };
  return (
    <Col
      className={`w-full rounded-[10px]  px-4 py-5 mt-[10px] ${theme === 'light' ? 'bg-neutral-50 border border-neutral-200' : 'bg-component-bg'}`}
    >
      <h3
        className={`text-[15px]/[20px] tracking-[0.5px] font-bold ${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
      >
        Latest News
      </h3>
      <Col className="gap-3.75">
        {news.map((item) => (
          <Col
            onClick={() => handleNewsNavigation(item.id)}
            key={`news-${item.id}`}
            className="py-2.5 cursor-pointer"
          >
            <h4
              className={`text-base/[25px] tracking-[0.5px] align-middle font-medium ${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
            >
              {item.title}
            </h4>
            <div
              className={`text-sm/[20px] align-middle font-light line-clamp-2 whitespace-normal ${theme === 'light' ? 'text-neutral-800' : 'text-white'}`}
              dangerouslySetInnerHTML={{
                __html: item.html_content || '',
              }}
            ></div>
          </Col>
        ))}
      </Col>
    </Col>
  );
}
