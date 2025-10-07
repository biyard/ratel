'use client';
import { Col } from '@/components/ui/col';
import { type NewsSummary } from '@/lib/api/models/home';
import { useTranslation } from 'react-i18next';
import { useRouter } from 'next/navigation';
import DisableBorderCard from './disable-border-card';
import DOMPurify from 'dompurify';

interface HomeNewsProps {
  newsData: NewsSummary[];
}

// Safe sanitization function that works in both server and client environments
const sanitizeHTML = (html: string): string => {
  // Check if we're in a browser environment
  if (typeof window !== 'undefined') {
    return DOMPurify.sanitize(html, {
      USE_PROFILES: { html: true },
    });
  }

  return html;
};

export default function HomeNews({ newsData }: HomeNewsProps) {
  const { t } = useTranslation('Home');
  const router = useRouter();

  const handleNewsNavigation = (id: number) => {
    router.push(`/news/${id}`);
  };

  // Don't render if no news data
  if (!newsData || newsData.length === 0) {
    return null;
  }

  return (
    <DisableBorderCard>
      <Col className="w-full rounded-[10px]">
        <h3 className="text-[15px]/[20px] tracking-[0.5px] font-bold text-text-primary">
          {t('latest_news')}
        </h3>
        <Col className="gap-3.75">
          {newsData.map((item) => (
            <Col
              onClick={() => handleNewsNavigation(item.id)}
              key={`news-${item.id}`}
              className="py-2.5 cursor-pointer"
            >
              <h4 className="text-base/[25px] tracking-[0.5px] align-middle font-medium text-text-primary">
                {item.title}
              </h4>
              {/* biome-ignore lint/security/noDangerouslySetInnerHtml: content is sanitized with DOMPurify */}
              <div
                className="text-sm/[20px] align-middle font-light line-clamp-2 whitespace-normal text-text-primary"
                dangerouslySetInnerHTML={{
                  __html: sanitizeHTML(item.html_content ?? ''),
                }}
              />
            </Col>
          ))}
        </Col>
      </Col>
    </DisableBorderCard>
  );
}
