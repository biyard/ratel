import { useState } from 'react';
import { State } from '@/types/state';
import { useNavigate } from 'react-router';
import { SpaceType } from '../types/space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import { useCreateSpaceMutation } from './use-create-space-mutation';
import { BoosterType } from '../types/booster-type';
import { showErrorToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { SPACE_DEFINITIONS } from '../types/space-definition';
import SpaceBoosterConfigModal from '../components/space-booster-config-modal';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class SpaceTypeSelectFormController {
  readonly spaceDefinitions: typeof SPACE_DEFINITIONS;

  constructor(
    public feed_id: string,
    public isLoading: State<boolean>,
    public selected: State<number>,
    public showConfigForm: State<boolean>,
    public navigate: ReturnType<typeof useNavigate>,
    public popup: ReturnType<typeof usePopup>,
    public createSpace: ReturnType<typeof useCreateSpaceMutation>,
    public t: TFunction<'Threads', undefined>,
  ) {
    this.spaceDefinitions = SPACE_DEFINITIONS;
  }

  handleCreateSpace = async ({
    spaceType,
    postPk,
    startedAt = null,
    endedAt = null,
    boosterType = null,
  }: {
    spaceType: SpaceType;
    postPk: string;
    userPk?: string;
    startedAt: number | null;
    endedAt: number | null;
    boosterType: BoosterType | null;
  }) => {
    if (this.isLoading.get()) return;
    this.isLoading.set(true);
    try {
      const { space_pk } = await this.createSpace.mutateAsync({
        postPk,
        spaceType: this.selectedSpace.type,
        startedAt,
        endedAt,
        booster: boosterType,
      });

      switch (spaceType) {
        case SpaceType.Poll:
          this.navigate(route.pollSpaceByPk(space_pk));
          break;
        default:
          this.navigate(route.deliberationSpaceById(space_pk));
      }
      this.popup.close();
    } catch {
      logger.error('Error creating space');
      showErrorToast('Failed to create space');
    } finally {
      this.isLoading.set(false);
    }
  };

  handleSelect = (idx: number) => {
    this.selected.set(idx);
  };

  handleBackToSelection = () => {
    this.showConfigForm.set(false);
  };

  get selectedSpace() {
    return this.spaceDefinitions[this.selected.get()];
  }

  handleNext = async () => {
    if (this.isLoading.get() || this.selected.get() === null) {
      return;
    }

    try {
      if (this.selectedSpace.canBoost) {
        this.popup
          .open(<SpaceBoosterConfigModal ctrl={this} />)
          .withTitle(this.t('select_space_type'));
        return;
      }

      this.isLoading.set(true);

      if (this.selectedSpace.canBoost) {
        this.showConfigForm.set(true);
      } else {
        await this.handleCreateSpace({
          spaceType: this.selectedSpace.type,
          postPk: this.feed_id,
          startedAt: null,
          endedAt: null,
          boosterType: null,
        });
      }
    } catch (error) {
      logger.error('Error in handleSend:', error);
      showErrorToast('Failed to process request');
    } finally {
      this.isLoading.set(false);
    }
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useSpaceTypeSelectFormController(feed_id: string) {
  const isLoading = useState(false);
  const selected = useState(0);
  const showConfigForm = useState(false);
  const navigate = useNavigate();
  const popup = usePopup();
  const createSpace = useCreateSpaceMutation();
  const { t } = useTranslation('Threads');

  return new SpaceTypeSelectFormController(
    feed_id,
    new State(isLoading),
    new State(selected),
    new State(showConfigForm),
    navigate,
    popup,
    createSpace,
    t,
  );
}
