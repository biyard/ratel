import { logger } from '@/lib/logger';
import { Col } from '@/components/ui/col';
import SpaceFileViewer from '@/features/spaces/components/file/viewer';
import { SpaceRecommendationPathProps } from '../space-recommendation-path-props';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import { useSpaceRecommendationViewerController } from './use-space-recommendation-viewer-controller';

export function SpaceRecommendationViewerPage({
  spacePk,
}: SpaceRecommendationPathProps) {
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
