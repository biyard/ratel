import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { useSpaceRecommendationEditorController } from './use-space-recommendation-editor-controller';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import SpaceFileEditors from '@/features/spaces/files/components/space-file-editor';
import SpaceFileViewer from '@/features/spaces/files/components/space-file-viewer';
import { SpacePathProps } from '@/features/space-path-props';

export function SpaceRecommendationEditorPage({ spacePk }: SpacePathProps) {
  const ctrl = useSpaceRecommendationEditorController(spacePk);
  const { t } = useTranslation('SpaceRecommendationEditor');

  return (
    <>
      <Col className="gap-8">
        <Col>
          <SpaceHTMLContentEditor
            htmlContent={ctrl.htmlContents.get()}
            canEdit={true}
            onContentChange={ctrl.handleUpdateContent}
            uploadAsset={ctrl.uploadAsset}
            url={null}
          />
        </Col>
        <Col className="gap-0">
          <Row className="gap-2 justify-end mb-4">
            {ctrl.editing.get() ? (
              <>
                <Button variant="primary" onClick={ctrl.handleFileSave}>
                  {t('btn_save')}
                </Button>
                <Button onClick={ctrl.handleDiscard}>{t('btn_discard')}</Button>
              </>
            ) : (
              <Button onClick={ctrl.handleEdit}>{t('btn_edit')}</Button>
            )}
          </Row>

          {ctrl.editing.get() ? (
            <SpaceFileEditors
              files={ctrl.files.get()}
              onremove={ctrl.handleRemoveFile}
              onadd={ctrl.handleAddFile}
            />
          ) : (
            <SpaceFileViewer files={ctrl.files.get()} />
          )}
        </Col>
      </Col>
    </>
  );
}
