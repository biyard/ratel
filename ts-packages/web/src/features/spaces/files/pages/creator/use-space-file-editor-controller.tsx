import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import useFileSpace from '../../hooks/use-file-space';
import { FileResponse } from '../../types/file-response';
import { Space } from '@/features/spaces/types/space';
import { useUpdateFileMutation } from '../../hooks/use-update-file-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import FileModel from '../../types/file';
import { useLinkFileMutation } from '../../hooks/use-file-links';
import { FileLinkTarget } from '../../types/file-link-target';

export class SpaceFileEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public file: FileResponse,
    public files: State<FileModel[]>,
    public editing: State<boolean>,
    public linkToOverview: State<boolean>,
    public updateFile: ReturnType<typeof useUpdateFileMutation>,
    public linkFile: ReturnType<typeof useLinkFileMutation>,
  ) {}

  handleEdit = () => {
    this.editing.set(true);
  };

  handleSave = async () => {
    const files = this.files.get();
    const shouldLinkToOverview = this.linkToOverview.get();

    try {
      await this.updateFile.mutateAsync({
        spacePk: this.spacePk,
        files,
      });

      // Link newly added files to Overview if checkbox was checked
      if (shouldLinkToOverview) {
        const fileUrlsToLink = files.filter((f) => f.url).map((f) => f.url!);

        for (const fileUrl of fileUrlsToLink) {
          try {
            await this.linkFile.mutateAsync({
              file_url: fileUrl,
              targets: [FileLinkTarget.Files, FileLinkTarget.Overview],
            });
          } catch (error) {
            console.error(`Failed to link file ${fileUrl}:`, error);
            // Continue with other files even if one fails
          }
        }
      }

      showSuccessToast('Success to update files');
    } catch {
      showErrorToast('Failed to update files');
    } finally {
      this.editing.set(false);
      this.linkToOverview.set(false); // Reset checkbox after save
    }
  };

  handleDiscard = () => {
    this.editing.set(false);
    this.linkToOverview.set(false); // Reset checkbox on discard
  };

  handleAddFile = (file: FileModel) => {
    this.files.set([...this.files.get(), file]);
  };

  handleRemoveFile = (index: number) => {
    const newFiles = this.files.get().filter((_, i) => i !== index);
    this.files.set(newFiles);
  };

  handleLinkToOverviewChange = (checked: boolean) => {
    this.linkToOverview.set(checked);
  };
}

export function useSpaceFileEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: file } = useFileSpace(spacePk);
  const files = useState(file.files || []);
  const editing = useState(false);
  const linkToOverview = useState(false);

  const updateFile = useUpdateFileMutation();
  const linkFile = useLinkFileMutation(spacePk);

  return new SpaceFileEditorController(
    spacePk,
    space,
    file,
    new State(files),
    new State(editing),
    new State(linkToOverview),
    updateFile,
    linkFile,
  );
}
