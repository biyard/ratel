import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceRecommendationViewerController } from './use-space-recommendation-viewer-controller';
import SpaceFileViewer from '@/features/spaces/files/components/space-file-viewer';
import { SpacePathProps } from '@/features/space-path-props';

export function SpaceRecommendationViewerPage({ spacePk }: SpacePathProps) {
  logger.debug(`SpaceRecommendationViewerPage: spacePk=${spacePk}`);

  const ctrl = useSpaceRecommendationViewerController(spacePk);

  return (
    <>
      <>
        <Col className="gap-8">
          <Col>
            <SpaceHTMLContentEditor
              htmlContent={ctrl.htmlContents}
              canEdit={false}
              onContentChange={() => {}}
              url={null}
            />
          </Col>
          <Col className="gap-0">
            <SpaceFileViewer files={ctrl.files} />
          </Col>
        </Col>
      </>
    </>
  );
}
