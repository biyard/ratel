import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { logger } from '@/lib/logger';

export default function DiscussionPage() {
  const { spacePk, discussionPk } = useParams<{
    spacePk: string;
    discussionPk: string;
  }>();
  const { data: space } = useSpaceById(spacePk);

  logger.debug('space pk, discussion pk: ', spacePk, discussionPk);

  if (!space) {
    throw new Error('Space not found');
  }

  return (
    <div className="bg-white text-black fixed top-0 left-0 flex flex-row w-full h-full">
      discussion page
    </div>
  );
}
