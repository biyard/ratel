import { useParams } from 'react-router';
import { usePollSpaceController } from './use-poll-space-controller';
import SpaceHeader from '@/features/spaces/components/header';
import Content from '../../[id]/poll/_components/content';
import SpaceContentEditor from '@/features/spaces/components/content-editor';

export default function PollSpacePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = usePollSpaceController(spacePk);

  return (
    <div className="flex flex-col w-full gap-6">
      <SpaceHeader {...ctrl.headerCtrl} />
      <SpaceContentEditor
        htmlContent={ctrl.headerCtrl.html_content}
        isEditMode={ctrl.isEditMode}
        onContentChange={ctrl.headerCtrl.updateContent}
      />
    </div>
  );
}
