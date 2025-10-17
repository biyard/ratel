import Card from '@/components/card';
import HtmlContentViewer from '@/components/html-content-viewer';

export default function TextViewer({ htmlContent }: { htmlContent: string }) {
  return (
    <>
      <Card>
        <HtmlContentViewer htmlContent={htmlContent} />
      </Card>
    </>
  );
}
