import { useState } from 'react';
import { State } from '@/types/state';
import { BoosterType } from '../../types/booster-type';
import { SpaceType } from '../../types/space-type';
import { useCreateSpaceMutation } from '../../hooks/use-create-space-mutation';
import { route } from '@/route';
import { showErrorToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { usePopup } from '@/lib/contexts/popup-service';
import { useNavigate } from 'react-router';
import SpaceCreateModal from '../space-type-selector-modal';
import { useTranslation } from 'react-i18next';

export class SpaceSettingModalController {
  constructor(
    public postId: string,
    public spaceType: SpaceType,
    public timezone: State<string>,
    public startTimestamp: State<number>,
    public endTimestamp: State<number>,
    public activateBooster: State<boolean>,
    public boosterType: State<BoosterType>,
    public createSpace: ReturnType<typeof useCreateSpaceMutation>,
    public navigate: ReturnType<typeof useNavigate>,
    public popup: ReturnType<typeof usePopup>,
    public t: ReturnType<typeof useTranslation>['t'],
    public isLoading: State<boolean>,
  ) {}

  handleStartTime = (timestamp: number) => {
    const delta = this.endTimestamp.get() - this.startTimestamp.get();
    this.startTimestamp.set(timestamp);
    this.endTimestamp.set(timestamp + delta);
  };

  handleEndTime = (timestamp: number) => {
    this.endTimestamp.set(timestamp);
  };
  handleBackToSelection = () => {
    this.popup
      .open(<SpaceCreateModal feed_id={this.postId} />)
      .withoutBackdropClose()
      .withTitle(this.t('select_space_type'));
  };

  handleCreate = async () => {
    this.isLoading.set(true);
    const startedAt = Math.floor(this.startTimestamp.get() / 1000);
    const endedAt = Math.floor(this.endTimestamp.get() / 1000);

    try {
      const { space_pk } = await this.createSpace.mutateAsync({
        postPk: this.postId,
        spaceType: this.spaceType,
        startedAt,
        endedAt,
        booster: this.boosterType.get(),
      });

      const url = route.spaceByPkAndType(space_pk, this.spaceType);
      this.navigate(url);
      this.popup.close();
    } catch {
      logger.error('Error creating space');
      showErrorToast('Failed to create space');
      this.isLoading.set(false);
    }
  };

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
    startedAt = Math.floor(startedAt / 1000);
    endedAt = Math.floor(endedAt / 1000);
    try {
      const { space_pk } = await this.createSpace.mutateAsync({
        postPk,
        spaceType: this.spaceType,
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
    }
  };
}

export function useSpaceSettingModalController(
  postId: string,
  spaceType: SpaceType,
) {
  const now = new Date();
  const hours = now.getHours();
  const startTime = new Date(now);
  startTime.setHours(hours + 1);
  startTime.setMinutes(0);
  startTime.setSeconds(0);
  startTime.setMilliseconds(0);

  const endTime = new Date(startTime.getTime() + 60 * 60 * 1000); // 1 hour later

  const timezone = useState<string>(
    Intl.DateTimeFormat().resolvedOptions().timeZone,
  );

  const activateBooster = useState<boolean>(false);

  const startTimestamp = useState<number>(startTime.getTime());
  const endTimestamp = useState<number>(endTime.getTime());
  const boosterType = useState<BoosterType>(BoosterType.NoBoost);
  const createSpace = useCreateSpaceMutation();
  const navigate = useNavigate();
  const popup = usePopup();
  const { t } = useTranslation('Threads');
  const isLoading = useState<boolean>(false);

  return new SpaceSettingModalController(
    postId,
    spaceType,
    new State(timezone),
    new State(startTimestamp),
    new State(endTimestamp),
    new State(activateBooster),
    new State(boosterType),
    createSpace,
    navigate,
    popup,
    t,
    new State(isLoading),
  );
}
