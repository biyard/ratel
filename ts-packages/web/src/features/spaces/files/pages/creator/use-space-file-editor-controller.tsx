import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState, useEffect } from 'react';
import useFileSpace from '../../hooks/use-file-space';
import { FileResponse } from '../../types/file-response';
import { Space } from '@/features/spaces/types/space';
import { useUpdateFileMutation } from '../../hooks/use-update-file-mutation';
import { useDeleteFileMutation } from '../../hooks/use-delete-file-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import FileModel from '../../types/file';

export class SpaceFileEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public file: FileResponse,
    public files: State<FileModel[]>,
    public originalFiles: State<FileModel[]>,
    public editing: State<boolean>,
    public updateFile: ReturnType<typeof useUpdateFileMutation>,
    public deleteFile: ReturnType<typeof useDeleteFileMutation>,
    public filesToDelete: State<string[]>,
  ) {}

  handleEdit = () => {
    // Store original files when entering edit mode
    this.originalFiles.set([...this.files.get()]);
    this.filesToDelete.set([]);
    this.editing.set(true);
  };

  handleSave = async () => {
    const currentFiles = this.files.get();
    const filesToDelete = this.filesToDelete.get();

    try {
      // First, delete files that were marked for deletion
      if (filesToDelete.length > 0) {
        await Promise.all(
          filesToDelete.map((fileUrl) =>
            this.deleteFile.mutateAsync({
              spacePk: this.spacePk,
              fileUrl,
            }),
          ),
        );
      }

      // Then update the files list
      await this.updateFile.mutateAsync({
        spacePk: this.spacePk,
        files: currentFiles,
      });

      showSuccessToast('Files updated successfully');
      this.filesToDelete.set([]);
    } catch (error) {
      showErrorToast('Failed to update files');
      console.error('Error updating files:', error);
    } finally {
      this.editing.set(false);
    }
  };

  handleDiscard = () => {
    // Restore original files
    this.files.set([...this.originalFiles.get()]);
    this.filesToDelete.set([]);
    this.editing.set(false);
  };

  handleAddFile = (file: FileModel) => {
    this.files.set([...this.files.get(), file]);
  };

  handleRemoveFile = (fileId: string) => {
    const currentFiles = this.files.get();
    const fileIndex = currentFiles.findIndex(file => file.id === fileId);
    
    if (fileIndex === -1) {
      console.warn('File not found for removal:', fileId);
      return;
    }
    
    const fileToRemove = currentFiles[fileIndex];

    // Just update UI locally - don't call API yet
    const newFiles = currentFiles.filter(file => file.id !== fileId);
    this.files.set(newFiles);

    // Track file URL for deletion on save (if it has a URL - uploaded files)
    if (fileToRemove?.url) {
      this.filesToDelete.set([...this.filesToDelete.get(), fileToRemove.url]);
    }
  };
}

export function useSpaceFileEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: file } = useFileSpace(spacePk);
  const files = useState<FileModel[]>(file.files || []);
  const originalFiles = useState<FileModel[]>([]);
  const filesToDelete = useState<string[]>([]);
  const editing = useState(false);

  const updateFile = useUpdateFileMutation();
  const deleteFile = useDeleteFileMutation();

  // Update local state when server data changes
  useEffect(() => {
    if (file?.files && !editing[0]) {
      files[1](file.files);
    }
  }, [file?.files, editing[0]]); // eslint-disable-line react-hooks/exhaustive-deps -- files[1] setter is stable

  return new SpaceFileEditorController(
    spacePk,
    space,
    file,
    new State(files),
    new State(originalFiles),
    new State(editing),
    updateFile,
    deleteFile,
    new State(filesToDelete),
  );
}
