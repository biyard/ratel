import { TiptapEditor } from '@/components/text-editor';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Input } from '@/components/ui/input';
import useSuspensePostById from '@/features/posts/hooks/use-post';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router';
import { useUpdateDraftMutation } from '@/features/posts/hooks/use-update-draft-mutation';
import { route } from '@/route';

export default function MyDraftEditPage() {
  const { postPk } = useParams<{ postPk: string }>();

  const { data: user } = useSuspenseUserInfo();
  const navigate = useNavigate();
  const {
    data: { post },
  } = useSuspensePostById(postPk);

  const [title, setTitle] = useState(post?.title || '');
  const [content, setContent] = useState(post?.html_contents || '');
  const updateDraft = useUpdateDraftMutation().mutateAsync;

  useEffect(() => {
    if (post) {
      setTitle(post.title);
      setContent(post.html_contents);
    }
  }, [post]);
  const handleSaveDraft = async () => {
    try {
      await updateDraft({
        postPk,
        title,
        content,
      });
    } catch (error) {
      console.error('Error updating draft:', error);
    }
  };

  const handleNext = async () => {
    await handleSaveDraft();
    // Navigate to preview or publish page
    navigate(route.threadByFeedId(postPk));
  };

  if (!user) {
    navigate('/');
    return null;
  }

  return (
    <Col>
      <Input value={title} onChange={(e) => setTitle(e.target.value)} />
      <TiptapEditor content={content} onUpdate={(value) => setContent(value)} />
      <Row>
        <Button variant="rounded_secondary" onClick={handleSaveDraft}>
          Save Draft
        </Button>
        <Button variant="rounded_primary" onClick={handleNext}>
          Save
        </Button>
      </Row>
    </Col>
  );
}
