import { logger } from '@/lib/logger';
import { SpaceFilePathProps } from '../space-file-path-props';
import { useSpaceFileEditorController } from './use-space-file-editor-controller';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import SpaceFileEditors from '@/features/spaces/components/file/editor';
import SpaceFileViewer from '@/features/spaces/components/file/viewer';

export function SpaceFileEditorPage({ spacePk }: SpaceFilePathProps) {
  logger.debug(`SpaceFileEditorPage: spacePk=${spacePk}`);

  const ctrl = useSpaceFileEditorController(spacePk);
  const { t } = useTranslation('SpaceFileEditor');

  return (
    <>
      <Col>
        <Row className="gap-2 justify-end mb-4">
          {ctrl.editing.get() ? (
            <>
              <Button variant="primary" onClick={ctrl.handleSave}>
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
    </>
  );
}
