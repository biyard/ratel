import { useState, useCallback, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { dataUrlToBlob, parseFileType } from '@/lib/file-utils';
import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { useUpdateDraftMutation } from '@/features/posts/hooks/use-update-draft-mutation';
import { useUpdateDraftImageMutation } from '@/features/posts/hooks/use-update-draft-image-mutation';
import { usePublishDraftMutation } from '@/features/posts/hooks/use-publish-draft-mutation';
import { State } from '@/types/state';
import { useCreatePostPageI18n } from './i18n';

import type { LexicalEditor, EditorState } from 'lexical';
import { SPACE_DEFINITIONS } from '@/features/spaces/types/space-definition';

const TITLE_MAX_LENGTH = 50;
const AUTO_SAVE_DELAY = 5000; // 5 seconds

export enum EditorStatus {
  Idle = 'idle',
  Saving = 'saving',
  Publishing = 'publishing',
}

export class CreatePostPageController {
  readonly TITLE_MAX_LENGTH = TITLE_MAX_LENGTH;
  readonly spaceDefinitions: typeof SPACE_DEFINITIONS;

  constructor(
    public postPk: State<string | null>,
    public title: State<string>,
    public content: State<string | null>,
    public image: State<string | null>,
    public skipCreatingSpace: State<boolean>,
    public spaceName: State<string>,
    public spaceDescription: State<string>,
    public status: State<EditorStatus>,
    public lastSavedAt: State<Date | null>,
    public isModified: State<boolean>,
    public selected: State<number>,
    public editorRef: React.RefObject<LexicalEditor | null>,
    public createPost: ReturnType<typeof useCreatePostMutation>['mutateAsync'],
    public updateDraft: ReturnType<
      typeof useUpdateDraftMutation
    >['mutateAsync'],
    public updateDraftImage: ReturnType<
      typeof useUpdateDraftImageMutation
    >['mutateAsync'],
    public publishDraft: ReturnType<
      typeof usePublishDraftMutation
    >['mutateAsync'],
    public navigate: ReturnType<typeof useNavigate>,
    public t: ReturnType<typeof useCreatePostPageI18n>,
  ) {
    logger.debug('CreatePostPageController initialized');
    this.spaceDefinitions = SPACE_DEFINITIONS;
  }

  get isPublishDisabled(): boolean {
    return (
      !this.title.get().trim() ||
      !this.content.get()?.trim() ||
      this.status.get() !== EditorStatus.Idle
    );
  }

  handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (value.length <= TITLE_MAX_LENGTH) {
      this.title.set(value);
      this.isModified.set(true);
    }
  };

  handleContentChange = (newContent: string) => {
    this.content.set(newContent);
    this.isModified.set(true);
  };

  handleSelectSpace = (idx: number) => {
    this.selected.set(idx);
  };

  handleImageUpload = async (imageUrl: string) => {
    const postPkValue = this.postPk.get();
    if (!postPkValue) return;

    try {
      const mime = imageUrl.match(/^data:([^;]+);base64,/);
      if (mime && mime[1]) {
        const res = await getPutObjectUrl(1, parseFileType(mime[1]));

        if (res && res.presigned_uris?.length > 0 && res.uris?.length > 0) {
          const blob = await dataUrlToBlob(imageUrl);
          await fetch(res.presigned_uris[0], {
            method: 'PUT',
            headers: {
              'Content-Type': mime[1],
            },
            body: blob,
          });
          const uploadedUrl = res.uris[0];

          await this.updateDraftImage({
            postPk: postPkValue,
            image: uploadedUrl,
          });
          this.image.set(uploadedUrl);
        }
      }
    } catch (error) {
      logger.error('Image upload failed:', error);
      showErrorToast(this.t.error_upload);
    }
  };

  handleRemoveImage = () => {
    this.image.set(null);
  };

  handlePublish = async () => {
    const postPkValue = this.postPk.get();
    if (
      !postPkValue ||
      !this.title.get().trim() ||
      !this.content.get()?.trim()
    ) {
      showErrorToast(this.t.error_empty_fields);
      return;
    }

    this.status.set(EditorStatus.Publishing);
    try {
      await this.publishDraft({
        postPk: postPkValue,
        title: this.title.get(),
        content: this.content.get(),
      });

      showSuccessToast(this.t.success_publish);
      this.navigate(route.threadByFeedId(postPkValue));
    } catch (error) {
      logger.error('Failed to publish post:', error);
      showErrorToast(this.t.error_publish);
      this.status.set(EditorStatus.Idle);
    }
  };

  autoSave = async () => {
    const postPkValue = this.postPk.get();
    if (
      !postPkValue ||
      !this.isModified.get() ||
      this.status.get() === EditorStatus.Saving
    ) {
      return;
    }

    this.status.set(EditorStatus.Saving);
    try {
      await this.updateDraft({
        postPk: postPkValue,
        title: this.title.get(),
        content: this.content.get() || '',
      });
      this.lastSavedAt.set(new Date());
      this.isModified.set(false);
    } catch (error) {
      logger.error('Auto-save failed:', error);
    } finally {
      this.status.set(EditorStatus.Idle);
    }
  };

  formatLastSaved = (date: Date | null): string => {
    if (!date) return '';
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    return `${this.t.last_saved_at} ${year}.${month}.${day} ${hours}:${minutes}`;
  };
}

export function useCreatePostPageController() {
  const navigate = useNavigate();
  const t = useCreatePostPageI18n();

  // State
  const postPk = useState<string | null>(null);
  const title = useState('');
  const content = useState<string | null>(null);
  const image = useState<string | null>(null);
  const skipCreatingSpace = useState(true);
  const spaceName = useState('');
  const spaceDescription = useState('');
  const status = useState<EditorStatus>(EditorStatus.Idle);
  const lastSavedAt = useState<Date | null>(null);
  const isModified = useState(false);
  const editorRef = useRef<LexicalEditor | null>(null);
  const selected = useState(0);

  // Mutations
  const { mutateAsync: createPost } = useCreatePostMutation();
  const { mutateAsync: updateDraft } = useUpdateDraftMutation();
  const { mutateAsync: updateDraftImage } = useUpdateDraftImageMutation();
  const { mutateAsync: publishDraft } = usePublishDraftMutation();

  const controller = new CreatePostPageController(
    new State(postPk),
    new State(title),
    new State(content),
    new State(image),
    new State(skipCreatingSpace),
    new State(spaceName),
    new State(spaceDescription),
    new State(status),
    new State(lastSavedAt),
    new State(isModified),
    new State(selected),
    editorRef,
    createPost,
    updateDraft,
    updateDraftImage,
    publishDraft,
    navigate,
    t,
  );

  // Initialize post on mount
  useEffect(() => {
    const initializePost = async () => {
      try {
        const result = await createPost({});
        controller.postPk.set(result.post_pk);
      } catch (error) {
        logger.error('Failed to initialize post:', error);
        showErrorToast(t.error_init);
      }
    };
    initializePost();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Auto-save
  useEffect(() => {
    const interval = setInterval(() => controller.autoSave(), AUTO_SAVE_DELAY);
    return () => clearInterval(interval);
  }, [controller]);

  return controller;
}
