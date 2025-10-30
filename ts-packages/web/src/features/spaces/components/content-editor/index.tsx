import Card from '@/components/card';
import { Edit1, Save } from '@/components/icons';

import { useState } from 'react';
import { PostEditor } from '@/features/posts/components/post-editor';
// import { executeOnKeyStroke } from '@/utils/key-event-handle';

export default function SpaceHTMLContentEditor({
  htmlContent,
  url,
  canEdit,
  // enableEnterToSave = false,
  onContentChange,
  onImageUpload,
  onRemoveImage,
}: {
  htmlContent: string;
  url: string | null;
  canEdit: boolean;
  enableEnterToSave?: boolean;
  onContentChange: (newContent: string) => void;
  onImageUpload: (imageUrl: string) => Promise<void>;
  onRemoveImage: () => Promise<void>;
}) {
  const [isEditing, setEditing] = useState<boolean>(false);
  const [content, setContent] = useState<string>(htmlContent);
  const Icon = !isEditing ? Edit1 : Save;

  // const onKeyDown = (e: React.KeyboardEvent) => {
  //   if (!isEditing || !enableEnterToSave) return;
  //   executeOnKeyStroke(
  //     e,
  //     () => {
  //       setEditing(false);
  //       onContentChange(content);
  //     },
  //     () => setEditing(false),
  //   );
  // };

  return (
    <>
      <Card className="relative pb-20">
        {canEdit && (
          <Icon
            role="button"
            className="absolute right-5 bottom-6 w-5 h-5 [&>path]:stroke-1  text-gray-400 cursor-pointer hover:text-gray-600"
            onClick={() => {
              if (isEditing) {
                console.log('save');
                onContentChange(content);
                setEditing(false);
              } else if (canEdit) {
                console.log('Start editing');
                setEditing(true);
              }
            }}
          />
        )}

        <PostEditor
          content={content}
          onUpdate={(nextContent) => {
            setContent(nextContent);
          }}
          editable={isEditing}
          showToolbar={isEditing}
          onImageUpload={onImageUpload}
          onRemoveImage={onRemoveImage}
          url={url}
          data-pw="space-recommendation-editor"
        />
      </Card>
    </>
  );
}
