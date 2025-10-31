import { useState } from 'react';
import { State } from '@/types/state';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';
import useMintSpaceArtworkMutation from '../../../hooks/use-mint-space-artwork-mutation';
import SpaceArtwork from '../../../types/space-artwork';

export class ArtTwinEditorController {
  constructor(
    public space: Space,
    public artwork: SpaceArtwork | null,
    public minting: State<boolean>,
    public t: TFunction<'ArtTwinEditor', undefined>,
    public mintMutation: ReturnType<typeof useMintSpaceArtworkMutation>,
  ) {}

  handleMint = async () => {
    if (this.artwork) {
      showErrorToast(this.t('artwork_already_minted'));
      return;
    }

    if (!this.space.permissions.has(TeamGroupPermission.SpaceEdit)) {
      showErrorToast(this.t('no_permission'));
      return;
    }

    this.minting.set(true);

    try {
      await this.mintMutation.mutateAsync({
        spacePk: this.space.pk,
      });
      showSuccessToast(this.t('mint_success'));
    } catch (error) {
      console.error('Failed to mint artwork:', error);
      showErrorToast(this.t('mint_error'));
    } finally {
      this.minting.set(false);
    }
  };
}

export function useArtTwinEditorController(
  space: Space,
  artwork: SpaceArtwork | null,
  t: TFunction<'ArtTwinEditor', undefined>,
) {
  const minting = useState(false);
  const mintMutation = useMintSpaceArtworkMutation();

  return new ArtTwinEditorController(
    space,
    artwork,
    new State(minting),
    t,
    mintMutation,
  );
}
