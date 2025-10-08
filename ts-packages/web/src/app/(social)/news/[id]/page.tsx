import { Suspense } from 'react';
import News from './_components/news';
import NewsHeader from './_components/header';
import { useParams } from 'react-router';

export default async function Page() {
  const params = useParams();
  const news_id = parseInt(params.id, 10);

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <div className="flex flex-col gap-6 w-full">
        <NewsHeader news_id={news_id} />
        <News news_id={news_id} />
      </div>
    </Suspense>
  );
}
