'use client';
import SpaceContents from '../../_components/space-contents';
import { useEditCoordinatorStore } from '../../space-store';
import useDagitBySpaceId from '@/hooks/use-dagit';

export default function ContentEditor({ spaceId }: { spaceId: number }) {
  const { isEdit, updateCommonData } = useEditCoordinatorStore();
  const { data: dAgit } = useDagitBySpaceId(spaceId);
  return (
    <SpaceContents
      isEdit={isEdit}
      htmlContents={dAgit?.html_contents ?? ''}
      setContents={(newContents) => {
        updateCommonData({ html_contents: newContents });
      }}
    />
  );
}
