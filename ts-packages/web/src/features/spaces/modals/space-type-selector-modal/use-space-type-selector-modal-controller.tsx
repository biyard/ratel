import { useState } from 'react';
import { State } from '@/types/state';
import { useNavigate } from 'react-router';
import { SpaceType } from '../../types/space-type';
import { usePopup } from '@/lib/contexts/popup-service';
import { useCreateSpaceMutation } from '../../hooks/use-create-space-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { route } from '@/route';
import { SPACE_DEFINITIONS } from '../../types/space-definition';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import SpaceCreateModal from '.';

export class SpaceTypeSelectModalController {
  readonly spaceDefinitions: typeof SPACE_DEFINITIONS;

  constructor(
    public feed_id: string,
    public isLoading: State<boolean>,
    public selected: State<number>,
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
  }: {
    spaceType: SpaceType;
    postPk: string;
  }) => {
    if (this.isLoading.get()) return;
    this.isLoading.set(true);

    try {
      const { space_pk } = await this.createSpace.mutateAsync({
        postPk,
        spaceType: this.selectedSpace.type,
      });

      this.navigate(route.spaceByType(spaceType, space_pk));

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
    this.popup
      .open(<SpaceCreateModal feed_id={this.feed_id} />)
      .withoutBackdropClose()
      .withTitle(this.t('select_space_type'));
  };

  get selectedSpace() {
    return this.spaceDefinitions[this.selected.get()];
  }

  handleNext = async () => {
    if (this.isLoading.get() || this.selected.get() === null) {
      return;
    }

    try {
      this.isLoading.set(true);

      await this.handleCreateSpace({
        spaceType: this.selectedSpace.type,
        postPk: this.feed_id,
      });
      showSuccessToast('Success to process request');
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

export function useSpaceTypeSelectModalController(feed_id: string) {
  const isLoading = useState(false);
  const selected = useState(0);
  const navigate = useNavigate();
  const popup = usePopup();
  const createSpace = useCreateSpaceMutation();
  const { t } = useTranslation('Threads');

  return new SpaceTypeSelectModalController(
    feed_id,
    new State(isLoading),
    new State(selected),
    navigate,
    popup,
    createSpace,
    t,
  );
}
