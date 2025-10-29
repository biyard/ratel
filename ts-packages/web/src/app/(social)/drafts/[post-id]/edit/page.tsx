import { TiptapEditor } from '@/components/text-editor';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Input } from '@/components/ui/input';
import useSuspensePostById from '@/features/posts/hooks/use-post';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate, useParams } from 'react-router';
import { useUpdateDraftMutation } from '@/features/posts/hooks/use-update-draft-mutation';
import { route } from '@/route';
import { Editor } from '@tiptap/core';
import { uploadAndReplaceImages } from '@/components/text-editor/image-utils';
import { useUploadBase64Image } from '@/hooks/use-upload-base64-image';
import { logger } from '@/lib/logger';
import { usePublishDraftMutation } from '@/features/posts/hooks/use-publish-draft-mutation';
import { showErrorToast } from '@/lib/toast';
import { validateContent, validateTitle } from '@/lib/valid-utils';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { FeedStatus } from '@/features/posts/types/post';

const validate = (t: TFunction, title: string, content: string): boolean => {
  if (!validateTitle(title)) {
    showErrorToast(t('title_validation_error'));
    return false;
  }
  if (!validateContent(content)) {
    showErrorToast(t('content_validation_error'));
    return false;
  }
  return true;
};

export default function MyDraftEditPage() {
  const { postPk } = useParams<{ postPk: string }>();

  const { data: user } = useSuspenseUserInfo();
  const navigate = useNavigate();
  const {
    data: { post },
  } = useSuspensePostById(postPk);
  const { t } = useTranslation('EditDraftPage');
  const [title, setTitle] = useState(post?.title || '');
  const [content, setContent] = useState(post?.html_contents || '');
  const [isModified, setIsModified] = useState(false);
  const [isDraftSaving, setDraftSaving] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const editorRef = useRef<Editor | null>(null);
  const updateDraft = useUpdateDraftMutation().mutateAsync;
  const publishDraft = usePublishDraftMutation().mutateAsync;
  const uploadBase64Image = useUploadBase64Image();

  const handleSaveDraft = useCallback(async () => {
    if (!isModified && isDraftSaving) {
      return;
    }
    const content = editorRef.current?.getHTML() || '';
    if (!validate(t, title, content)) return;
    try {
      setDraftSaving(true);
      await updateDraft({
        postPk,
        title,
        content,
      });
      setIsModified(false);
      setDraftSaving(false);
    } catch (error) {
      console.error('Error updating draft:', error);
    }
  }, [isModified, isDraftSaving, t, title, updateDraft, postPk]);

  const handleNext = async () => {
    if (isSaving) return;
    const content = editorRef.current?.getHTML() || '';
    if (!validate(t, title, content)) return;

    try {
      setIsSaving(true);
      const uploadedUrls = await uploadAndReplaceImages(
        editorRef.current,
        uploadBase64Image,
      );
      const updatedContent = editorRef.current.getHTML();

      logger.debug('Uploaded S3 URLs:', uploadedUrls);

      await publishDraft({
        postPk,
        title,
        content: updatedContent,
        imageUrls: uploadedUrls,
      });

      navigate(route.threadByFeedId(postPk), {
        replace: true,
      });
    } catch (error) {
      logger.error('Error updating draft:', error);
    } finally {
      setIsSaving(false);
    }
  };

  useEffect(() => {
    const timeoutId = setTimeout(async () => {
      if (validateContent(content) && validateTitle(title)) {
        await handleSaveDraft();
      } else {
        logger.debug('Draft not saved due to validation errors.');
      }
    }, 10000);
    return () => clearTimeout(timeoutId);
  }, [content, handleSaveDraft, title]);

  useEffect(() => {
    if (post.status !== FeedStatus.Draft) {
      navigate(route.threadByFeedId(postPk), {
        replace: true,
      });
    }
  }, [post.status, postPk, navigate]);

  if (!user) {
    navigate('/');
    return null;
  }

  return (
    <Col>
      <Input
        id="post-title-input"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <TiptapEditor
        ref={editorRef}
        content={content}
        onUpdate={() => {
          setContent(editorRef.current?.getHTML() || '');
          setIsModified(true);
        }}
        data-pw="post-content-editor"
      />
      <Row>
        <Button
          id="save-draft-button"
          variant="rounded_secondary"
          onClick={handleSaveDraft}
          disabled={isSaving}
        >
          Save Draft
        </Button>
        <Button
          id="publish-post-button"
          variant="rounded_primary"
          onClick={handleNext}
          disabled={isSaving}
        >
          {isSaving ? 'Saving...' : 'Save'}
        </Button>
      </Row>
    </Col>
  );
}
