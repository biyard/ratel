import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import useFileSpace from '../../hooks/use-file-space';
import { FileResponse } from '../../types/file-response';
import { Space } from '@/features/spaces/types/space';
import { useUpdateFileMutation } from '../../hooks/use-update-file-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import FileModel from '../../types/file';
import { deleteSpaceFile } from '@/lib/api/ratel/spaces.v3';

export class SpaceFileEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public file: FileResponse,
    public files: State<FileModel[]>,
    public editing: State<boolean>,
    public updateFile: ReturnType<typeof useUpdateFileMutation>,
  ) {}

  handleEdit = () => {
    this.editing.set(true);
  };

  handleSave = async () => {
    const files = this.files.get();

    try {
      await this.updateFile.mutateAsync({
        spacePk: this.spacePk,
        files,
      });

      showSuccessToast('Success to update files');
    } catch {
      showErrorToast('Failed to update files');
    } finally {
      this.editing.set(false);
    }
  };

  handleDiscard = () => {
    this.editing.set(false);
  };

  handleAddFile = (file: FileModel) => {
    this.files.set([...this.files.get(), file]);
  };

  handleRemoveFile = async (index: number) => {
    const currentFiles = this.files.get();
    const fileToRemove = currentFiles[index];
    
    // Optimistically update UI
    const newFiles = currentFiles.filter((_, i) => i !== index);
    this.files.set(newFiles);

    // Call API to delete file and cascade to Overview/Boards
    if (fileToRemove?.url) {
      try {
        await deleteSpaceFile(this.spacePk, fileToRemove.url);
        showSuccessToast('File deleted successfully');
      } catch (error) {
        showErrorToast('Failed to delete file');
        // Revert on error
        this.files.set(currentFiles);
      }
    }
  };
}

export function useSpaceFileEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: file } = useFileSpace(spacePk);
  const files = useState(file.files || []);
  const editing = useState(false);

  const updateFile = useUpdateFileMutation();

  return new SpaceFileEditorController(
    spacePk,
    space,
    file,
    new State(files),
    new State(editing),
    updateFile,
  );
}
