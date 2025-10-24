import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import useFileSpace from '../../hooks/use-file-space';
import { FileResponse } from '../../types/file-response';
import { Space } from '@/features/spaces/types/space';
import { useUpdateFileMutation } from '../../hooks/use-update-file-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import FileType from '../../types/file';

export class SpaceFileEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public file: FileResponse,
    public files: State<FileType[]>,
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

  handleAddFile = (file: FileType) => {
    this.files.set([...this.files.get(), file]);
  };

  handleRemoveFile = (index: number) => {
    const newFiles = this.files.get().filter((_, i) => i !== index);
    this.files.set(newFiles);
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
