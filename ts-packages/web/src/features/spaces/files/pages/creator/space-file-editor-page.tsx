import { useSpaceFileEditorController } from './use-space-file-editor-controller';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import SpaceFileEditors from '../../components/space-file-editor';
import SpaceFileViewer from '../../components/space-file-viewer';
import { SpacePathProps } from '@/features/space-path-props';
import { useState } from 'react';
import { FileLinkTarget, isTargetMatch } from '../../types/file-link-target';
import { useListFileLinks } from '../../hooks/use-file-links';
import FileModel from '../../types/file';

type FileTab = 'all' | 'overview' | 'boards';

export function SpaceFileEditorPage({ spacePk }: SpacePathProps) {
  const ctrl = useSpaceFileEditorController(spacePk);
  const { t } = useTranslation('SpaceFileEditor');
  const [activeTab, setActiveTab] = useState<FileTab>('all');
  const { data: fileLinks } = useListFileLinks(spacePk);

  const getFilteredFiles = (): FileModel[] => {
    const allFiles = ctrl.files.get();

    if (activeTab === 'all') {
      return allFiles;
    }

    if (!fileLinks?.file_links) {
      return [];
    }

    const targetFilter =
      activeTab === 'overview' ? FileLinkTarget.Overview : FileLinkTarget.Board;

    const linkedUrls = fileLinks.file_links
      .filter((link) => {
        return isTargetMatch(link.link_target, targetFilter);
      })
      .map((link) => link.file_url);

    return allFiles.filter((file) => file.url && linkedUrls.includes(file.url));
  };

  const filteredFiles = getFilteredFiles();

  const displayedFiles = activeTab === 'all' ? ctrl.files.get() : filteredFiles;

  return (
    <Col>
      <Row className="justify-between items-center mb-4">
        <Row className="gap-2">
          <Button
            variant={activeTab === 'all' ? 'primary' : 'default'}
            onClick={() => setActiveTab('all')}
          >
            {t('tab_all_files')}
          </Button>
          <Button
            variant={activeTab === 'overview' ? 'primary' : 'default'}
            onClick={() => setActiveTab('overview')}
          >
            {t('tab_overview')}
          </Button>
          <Button
            variant={activeTab === 'boards' ? 'primary' : 'default'}
            onClick={() => setActiveTab('boards')}
          >
            {t('tab_boards')}
          </Button>
        </Row>

        <Row className="gap-2 justify-end">
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
      </Row>

      {ctrl.editing.get() ? (
        <SpaceFileEditors
          files={displayedFiles}
          onremove={ctrl.handleRemoveFile}
          onadd={ctrl.handleAddFile}
        />
      ) : (
        <SpaceFileViewer files={displayedFiles} />
      )}
    </Col>
  );
}
