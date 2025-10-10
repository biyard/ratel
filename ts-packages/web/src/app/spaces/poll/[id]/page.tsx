import { useParams } from 'react-router';
import useSpaceById from '@/hooks/use-space-by-id';
import { PollHandler } from './handlers/poll-handler';
import SpaceHeader from '../../_components/space-header';
import { useSpaceEditStore } from '../../_components/space-edit-store';
import { useEffect } from 'react';
import { PublishingScope } from '@/lib/api/models/notice';

export default function PollSpacePage() {
  const { spaceId } = useParams<{ spaceId: string }>();
  const { data: space, isLoading } = useSpaceById(parseInt(spaceId!));
  const { setHandler } = useSpaceEditStore();

  useEffect(() => {
    if (space) {
      // SpaceCommon 형태로 변환 (임시 - 필요한 필드만)
      const spaceCommon = {
        pk: space.id.toString(),
        sk: '', // 임시
        created_at: space.created_at,
        updated_at: space.updated_at,
        post_pk: '', // 임시
        title: space.title || '',
        author_username: space.owner_username,
        publish_state: space.status === 1 ? 'DRAFT' : 'PUBLISHED', // SpaceStatus.Draft = 1
        visibility:
          space.publishing_scope === PublishingScope.Public
            ? 'PUBLIC'
            : 'PRIVATE',
      } as any; // 타입 에러 임시 해결

      const handler = new PollHandler(spaceCommon);
      setHandler(handler);
    }
  }, [space, setHandler]);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (!space) {
    return <div>Space not found</div>;
  }

  return (
    <div className="flex flex-col w-full gap-6">
      <SpaceHeader
        handler={
          new PollHandler({
            pk: space.pk.toString(),
            title: space.title,
            author_username: space.author_username,
            publish_state: space.status === 'DRAFT' ? 'DRAFT' : 'PUBLISHED',
            visibility:
              space.publishing_scope === 'PUBLIC' ? 'PUBLIC' : 'PRIVATE',
          })
        }
      />

      <div className="flex flex-col gap-4">
        <h1>{space.title}</h1>
        <p>Poll Space Content</p>
        {/* Poll 컴포넌트들 추가 */}
      </div>
    </div>
  );
}
