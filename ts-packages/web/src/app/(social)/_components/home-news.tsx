import { Col } from '@/components/ui/col';
import { NewsSummary } from '@/lib/api/models/home';
import { useTranslations } from 'next-intl';
import { useRouter } from 'next/navigation';
import React from 'react';

interface HomeNewsProps {
  newsData: NewsSummary[];
}

export default function HomeNews({ newsData }: HomeNewsProps) {
  const t = useTranslations('Home');
  const router = useRouter();

  const handleNewsNavigation = (id: number) => {
    router.push(`/news/${id}`);
  };

  // Don't render if no news data
  if (!newsData || newsData.length === 0) {
    return null;
  }

  return (
    <Col className="w-full rounded-[10px] bg-card px-4 py-5 mt-[10px]">
      <h3 className="text-[15px]/[20px] tracking-[0.5px] font-bold text-foreground">
        {t('latest_news')}
      </h3>
      <Col className="gap-3.75">
        {newsData.map((item) => (
          <Col
            onClick={() => handleNewsNavigation(item.id)}
            key={`news-${item.id}`}
            className="py-2.5 cursor-pointer"
          >
            <h4 className="text-base/[25px] tracking-[0.5px] align-middle font-medium text-foreground">
              {item.title}
            </h4>
            <div
              className="text-sm/[20px] align-middle font-light line-clamp-2 whitespace-normal text-card-meta"
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
