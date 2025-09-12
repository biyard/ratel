'use client';
import useSpaceById from '@/hooks/use-space-by-id';
import SpaceContents from '../../_components/space-contents';
import { useEditCoordinatorStore } from '../../space-store';

export default function Content({ spaceId }: { spaceId: number }) {
  const { isEdit, updateCommonData } = useEditCoordinatorStore();
  const { data: space } = useSpaceById(spaceId);
  return (
    <SpaceContents
      isEdit={isEdit}
      htmlContents={space?.html_contents ?? ''}
      setContents={(newContents) => {
        updateCommonData({ html_contents: newContents });
      }}
    />
  );
}
