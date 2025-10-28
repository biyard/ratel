import { TiptapEditor } from '@/components/text-editor';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { usePostById } from '@/features/posts/hooks/use-post';
import { useState } from 'react';

export default function EditDraftPage({ postPk }: { postPk?: string }) {
  const {
    data: { post },
  } = usePostById(postPk);
  const [title, setTitle] = useState<string>(post?.title || '');
  const [content, setContent] = useState<string>(post?.html_contents || '');
  return (
    <div className="flex flex-col gap-4 p-4">
      <Input
        placeholder="Post Title"
        className="w-full"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <TiptapEditor
        placeholder="Type your script"
        editable={true}
        showToolbar={true}
        content={content}
        onUpdate={(nextContent) => {
          setContent(nextContent);
        }}
      />
      <div>
        <Button onClick={() => console.log('Post created!')}>Next</Button>
      </div>
    </div>
  );
}
