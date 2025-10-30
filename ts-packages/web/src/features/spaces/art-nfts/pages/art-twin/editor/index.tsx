import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import Card from '@/components/card';
import { SpacePathProps } from '@/features/space-path-props';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useArtTwinEditorController } from './use-art-twin-editor-controller';
import { i18nArtTwinEditor } from './art-twin-editor-i18n';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';
import useSuspenseSpaceArtwork from '../../../hooks/use-space-artwork';

export default function SpaceArtNftArtTwinEditorPage({
  spacePk,
}: SpacePathProps) {
  const { data: space } = useSpaceById(spacePk);
  const { data: artwork } = useSuspenseSpaceArtwork(spacePk);
  const { t, i18n } = useTranslation('ArtTwinEditor');
  // Register i18n
  if (!i18n.hasResourceBundle(i18n.language, 'ArtTwinEditor')) {
    i18n.addResourceBundle(
      'en',
      'ArtTwinEditor',
      i18nArtTwinEditor.en,
      true,
      true,
    );
    i18n.addResourceBundle(
      'ko',
      'ArtTwinEditor',
      i18nArtTwinEditor.ko,
      true,
      true,
    );
  }

  if (!space) {
    throw new Error('Space not found');
  }

  const ctrl = useArtTwinEditorController(space, artwork, t);

  return (
    <Card variant="secondary">
      <Col className="gap-4">
        {!artwork && space.permissions.has(TeamGroupPermission.SpaceEdit) && (
          <Row className="justify-center">
            <Button
              variant="primary"
              onClick={ctrl.handleMint}
              disabled={ctrl.minting.get() || ctrl.mintMutation.isPending}
            >
              {ctrl.minting.get() || ctrl.mintMutation.isPending
                ? t('minting')
                : t('mint_button')}
            </Button>
          </Row>
        )}

        {artwork && (
          <Col className="gap-2">
            <div className="text-sm text-gray-500">
              Contract: {artwork.contractAddress}
            </div>
            <div className="text-sm text-gray-500">
              Metadata URI: {artwork.metadataUri}
            </div>
            <div className="text-sm text-gray-500">
              Created: {new Date(artwork.createdAt).toLocaleString()}
            </div>
          </Col>
        )}
      </Col>
    </Card>
  );
}
