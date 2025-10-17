import Card from '@/components/card';
import HtmlContentViewer from '@/components/html-content-viewer';
import { Edit1, Save } from '@/components/icons';
import TextEditor from '@/components/text-editor/text-editor';
import { useState } from 'react';
import TextViewer from './text-viewer';
import { executeOnKeyStroke } from '@/utils/key-event-handle';

export default function SpaceHTMLContentEditor({
  htmlContent,
  canEdit,
  onContentChange,
}: {
  htmlContent: string;
  canEdit: boolean;
  onContentChange: (newContent: string) => void;
}) {
  const [editable, setEditable] = useState(false);
  const [content, setContent] = useState(htmlContent);

  if (!canEdit) {
    return <TextViewer htmlContent={htmlContent} />;
  }

  const Icon = !editable ? Edit1 : Save;
  const onClick = () => {
    if (editable) {
      onContentChange(content);
    }
    setEditable(!editable);
  };
  const onKeyDown = (e: React.KeyboardEvent) => {
    if (!editable) return;
    executeOnKeyStroke(
      e,
      () => {
        onContentChange(content);
        setEditable(false);
      },
      () => setEditable(false),
    );
  };

  return (
    <>
      <Card className="relative">
        <Icon
          role="button"
          className="absolute top-3 right-3 w-5 h-5 [&>path]:stroke-1  text-gray-400 cursor-pointer hover:text-gray-600"
          onClick={onClick}
        />
        {editable ? (
          <TextEditor
            content={content}
            onChange={setContent}
            onKeyDown={onKeyDown}
          />
        ) : (
          <HtmlContentViewer htmlContent={htmlContent} />
        )}
      </Card>
    </>
  );
}
