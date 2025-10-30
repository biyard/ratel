import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceLayoutContext } from './use-space-layout-context';
import FileEditor from '@/features/spaces/components/media-file-editor';
import SpaceFileViewer from '@/features/spaces/files/components/space-file-viewer';

export function SpaceHomePage() {
  const ctrl = useSpaceLayoutContext();

  return (
    <>
      <div className="flex flex-col w-full gap-2.5">
        <SpaceHTMLContentEditor
          htmlContent={ctrl.space.content}
          canEdit={ctrl.isAdmin}
          onContentChange={ctrl.handleChange}
        />

        {ctrl.isAdmin ? (
          <FileEditor
            t={ctrl.t}
            files={ctrl.space.files}
            onremove={ctrl.handleRemoveFile}
            onadd={ctrl.handleAddFile}
          />
        ) : (
          <SpaceFileViewer files={ctrl.space.files} />
        )}
      </div>
    </>
  );
}
