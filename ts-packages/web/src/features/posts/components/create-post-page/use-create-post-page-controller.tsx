import { useState, useCallback, useEffect, useRef } from 'react';
import {
  useLoaderData,
  useNavigate,
  useParams,
  useSearchParams,
} from 'react-router';
import { route } from '@/route';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { dataUrlToBlob, parseFileType } from '@/lib/file-utils';
import { getPutObjectUrl } from '@/lib/api/ratel/assets.v3';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { useUpdateDraftMutation } from '@/features/posts/hooks/use-update-draft-mutation';
import { useUpdateDraftImageMutation } from '@/features/posts/hooks/use-update-draft-image-mutation';
import { usePublishDraftMutation } from '@/features/posts/hooks/use-publish-draft-mutation';
import { getPost } from '@/features/posts/hooks/use-post';
import { State } from '@/types/state';
import { useCreatePostPageI18n } from './i18n';

import type { LexicalEditor, EditorState } from 'lexical';
import { SPACE_DEFINITIONS } from '@/features/spaces/types/space-definition';
import { stripHtml } from '@/lib/string-filter-utils';

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
    public teamPk: State<string | null>,
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
    public previousTitle: State<string>,
    public previousContent: State<string>,
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
    this.spaceDefinitions = SPACE_DEFINITIONS;
  }

  get isPublishDisabled(): boolean {
    return (
      !this.title.get().trim() ||
      !this.content.get()?.trim() ||
      this.status.get() !== EditorStatus.Idle
    );
  }

  get actionButtonText(): string {
    if (!this.skipCreatingSpace.get()) {
      return this.t.btn_next;
    }

    return this.t.publish;
  }

  detectChanged = () => {
    const titleChanged = this.previousTitle.get() !== this.title.get();
    const contentChanged = this.previousContent.get() !== this.content.get();

    return titleChanged || contentChanged;
  };

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

  handleSubmit = async () => {
    if (this.skipCreatingSpace.get()) {
      return this.handlePublish();
    }
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
    // strip html tags
    const contents = stripHtml(this.content.get() || '').trim();
    const title = stripHtml(this.title.get()).trim();
    logger.debug('Auto-saving draft:', {
      title: this.title.get(),
      content: contents,
      realContent: this.content.get(),
    });

    if (title === '' && contents === '') {
      logger.debug('Both title and content are empty. Skipping auto-save.');
      return;
    }

    if (
      this.previousTitle.get() === this.title.get() &&
      this.previousContent.get() === this.content.get()
    ) {
      logger.debug(
        'No changes detected since last auto-save. Skipping auto-save.',
      );
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
      this.previousTitle.set(this.title.get());
      this.previousContent.set(this.content.get() || '');
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
    const seconds = String(date.getSeconds()).padStart(2, '0');
    return `${this.t.last_saved_at} ${year}.${month}.${day} ${hours}:${minutes}:${seconds}`;
  };
}

export function useCreatePostPageController() {
  const navigate = useNavigate();
  const t = useCreatePostPageI18n();
  const [searchParams] = useSearchParams();
  const postPkParam = searchParams.get('post-pk');
  const teamPkParam = searchParams.get('team-pk');

  // State
  const postPk = useState<string | null>(postPkParam);
  const teamPk = useState<string | null>(teamPkParam);
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
  const previousTitle = useState('');
  const previousContent = useState<string>('');

  // Mutations
  const { mutateAsync: createPost } = useCreatePostMutation();
  const { mutateAsync: updateDraft } = useUpdateDraftMutation();
  const { mutateAsync: updateDraftImage } = useUpdateDraftImageMutation();
  const { mutateAsync: publishDraft } = usePublishDraftMutation();

  const controller = new CreatePostPageController(
    new State(postPk),
    new State(teamPk),
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
    new State(previousTitle),
    new State(previousContent),
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
      // If postPk is already set from query params, fetch existing post data
      if (postPkParam) {
        logger.debug('Using existing postPk from query params:', postPkParam);
        try {
          const postData = await getPost(postPkParam);
          controller.title.set(postData.post.title);
          controller.content.set(postData.post.html_contents);
          controller.previousTitle.set(postData.post.title);
          controller.previousContent.set(postData.post.html_contents);

          // Set image if exists (first URL in the urls array)
          if (postData.post.urls && postData.post.urls.length > 0) {
            controller.image.set(postData.post.urls[0]);
          }

          // Mark as not modified since we just loaded from server
          controller.isModified.set(false);
          controller.lastSavedAt.set(new Date(postData.post.updated_at * 1000));
        } catch (error) {
          logger.error('Failed to fetch post data:', error);
          showErrorToast(t.error_init);
        }
        return;
      }

      // Create new post if no postPk provided
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
