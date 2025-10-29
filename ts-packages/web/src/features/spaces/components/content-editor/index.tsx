import Card from '@/components/card';
import { Edit1, Save } from '@/components/icons';

import { useState } from 'react';
import { TiptapEditor } from '@/components/text-editor';
// import { executeOnKeyStroke } from '@/utils/key-event-handle';

export default function SpaceHTMLContentEditor({
  htmlContent,
  canEdit,
  // enableEnterToSave = false,
  onContentChange,
}: {
  htmlContent: string;
  canEdit: boolean;
  enableEnterToSave?: boolean;
  onContentChange: (newContent: string) => void;
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
      <Card className="relative">
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
        <TiptapEditor
          content={content}
          onUpdate={(nextContent) => {
            setContent(nextContent);
          }}
          editable={isEditing}
          showToolbar={isEditing}
          data-pw="space-recommendation-editor"
        />
      </Card>
    </>
  );
}
