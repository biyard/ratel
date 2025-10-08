import { Col } from '@/components/ui/col';
import { useTranslation } from 'react-i18next';
import DisableBorderCard from './disable-border-card';
import { useNews } from '../_hooks/use-news';
import { useNavigate } from 'react-router';

export interface NewsItem {
  id: number;
  title: string;
  html_content: string;
  created_at: number;
}

export default function Wrapper() {
  return <News />;
}

function News() {
  const { t } = useTranslation('Home');
  const router = useNavigate();
  const news = useNews().data.items;

  const handleNewsNavigation = (id: number) => {
    router(`/news/${id}`);
  };

  if (!news || news.length === 0) {
    return <div></div>;
  }

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
