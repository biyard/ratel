import Card from '@/components/card';
import { Edit1, Save } from '@/components/icons';

import { useState } from 'react';
import { TiptapEditor } from '@/components/text-editor';

export default function SpaceHTMLContentEditor({
  htmlContent,
  canEdit,
  onContentChange,
}: {
  htmlContent: string;
  canEdit: boolean;
  onContentChange: (newContent: string) => void;
}) {
  const [isEditing, setEditing] = useState<boolean>(false);
  const [content, setContent] = useState<string>(htmlContent);
  const Icon = !isEditing ? Edit1 : Save;

  return (
    <>
      <Card className="relative">
        {canEdit && (
          <Icon
            role="button"
            className="absolute right-5 bottom-6 w-5 h-5 [&>path]:stroke-1  text-gray-400 cursor-pointer hover:text-gray-600"
            onClick={() => {
              if (isEditing) {
                onContentChange(content);
                setEditing(false);
              } else if (canEdit) {
                setEditing(true);
              }
            }}
          />
        )}
        <TiptapEditor
          content={content}
          onUpdate={(nextContent) => {
            setContent(nextContent);
          }}
          editable={isEditing}
          showToolbar={isEditing}
        />
      </Card>
    </>
  );
}
