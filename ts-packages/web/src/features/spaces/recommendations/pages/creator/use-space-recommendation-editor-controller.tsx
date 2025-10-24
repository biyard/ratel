import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import { Space } from '@/features/spaces/types/space';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import useRecommendationSpace from '../../hooks/use-recommendation-space';
import { useUpdateRecommendationContentMutation } from '../../hooks/use-update-recommendation-content-mutation';
import { useUpdateRecommendationFileMutation } from '../../hooks/use-update-recommendation-file-mutation';
import { SpaceRecommendationResponse } from '../../types/recommendation-response';
import FileModel from '@/features/spaces/files/types/file';

export class SpaceRecommendationEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public recommendation: SpaceRecommendationResponse,
    public files: State<FileModel[]>,
    public htmlContents: State<string>,
    public editing: State<boolean>,
    public updateContent: ReturnType<
      typeof useUpdateRecommendationContentMutation
    >,
    public updateFile: ReturnType<typeof useUpdateRecommendationFileMutation>,
  ) {}

  handleEdit = () => {
    this.editing.set(true);
  };

  handleContentSave = async (htmlContents: string) => {
    try {
      await this.updateContent.mutateAsync({
        spacePk: this.spacePk,
        htmlContents,
      });

      showSuccessToast('Success to update recommendations');
    } catch {
      showErrorToast('Failed to update recommendations');
    } finally {
      this.editing.set(false);
    }
  };

  handleFileSave = async () => {
    const files = this.files.get();

    try {
      await this.updateFile.mutateAsync({
        spacePk: this.spacePk,
        files,
      });

      showSuccessToast('Success to update recommendations');
    } catch {
      showErrorToast('Failed to update recommendations');
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

  handleRemoveFile = (index: number) => {
    const newFiles = this.files.get().filter((_, i) => i !== index);
    this.files.set(newFiles);
  };

  handleUpdateContent = async (htmlContents: string) => {
    this.htmlContents.set(htmlContents);
    this.handleContentSave(htmlContents);
  };
}

export function useSpaceRecommendationEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: recommendation } = useRecommendationSpace(spacePk);
  const files = useState(recommendation.files || []);
  const htmlContents = useState(recommendation.html_contents || '');
  const editing = useState(false);

  const updateContent = useUpdateRecommendationContentMutation();
  const updateFile = useUpdateRecommendationFileMutation();

  return new SpaceRecommendationEditorController(
    spacePk,
    space,
    recommendation,
    new State(files),
    new State(htmlContents),
    new State(editing),
    updateContent,
    updateFile,
  );
}
