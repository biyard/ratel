import { useNewsByID } from '@/app/notifications/_hook/use-news';
import { TiptapEditor } from '@/components/text-editor';
import Card from '@/components/card';

export default function News({ news_id }: { news_id: number }) {
  const { data: news } = useNewsByID(news_id);

  return (
    <div className="flex flex-col w-full gap-2.5">
      <Card variant="secondary">
        <div className="flex flex-col gap-5">
          <TiptapEditor
            editable={false}
            showToolbar={false}
            content={news?.html_content || ''}
          />
          {news?.user_id && (
            <div className="relative w-full h-72 rounded-[10px] overflow-hidden">
              <img
                className="object-cover w-full h-full"
                src={''}
                alt={news.title || 'Post Image'}
              />
            </div>
          )}
        </div>
      </Card>
    </div>
  );
}
