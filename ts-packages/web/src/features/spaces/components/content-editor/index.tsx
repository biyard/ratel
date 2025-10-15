import Card from '@/components/card';
import HtmlContentViewer from '@/components/html-content-viewer';
import TextEditor from '@/components/text-editor/text-editor';

export default function SpaceHTMLContentEditor({
  htmlContent,
  isEditMode,
  onContentChange,
}: {
  htmlContent: string;
  isEditMode: boolean;
  onContentChange: (newContent: string) => void;
}) {
  return (
    <>
      {isEditMode ? (
        <TextEditor content={htmlContent} onChange={onContentChange} />
      ) : (
        <Card>
          <HtmlContentViewer htmlContent={htmlContent} />
        </Card>
      )}
    </>
  );
}
