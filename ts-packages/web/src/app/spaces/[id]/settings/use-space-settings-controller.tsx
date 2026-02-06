import { useState } from 'react';
import { useSpaceSettingsData } from './use-space-settings-data';
import { State } from '@/types/state';
import { useSpaceUpdateAnonymousParticipationMutation } from '@/features/spaces/hooks/use-space-update-anonymous-participation-mutation';
import { logger } from '@/lib/logger';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { useSettingsI18n } from './i18n';
import { useSpaceUpdateChangeVisibilityMutation } from '@/features/spaces/hooks/use-space-update-change-visibility-mutation';
import { usePublishSpaceMutation } from '@/features/spaces/hooks/use-publish-mutation';

export class SpaceSettingsController {
  constructor(
    public data: ReturnType<typeof useSpaceSettingsData>,
    public state: State<boolean>,
    public publishSpace: ReturnType<typeof usePublishSpaceMutation>,
    public updateAnonymousParticipation: ReturnType<
      typeof useSpaceUpdateAnonymousParticipationMutation
    >,
    public updateChangeVisibility: ReturnType<
      typeof useSpaceUpdateChangeVisibilityMutation
    >,
    public spacePk: string,
    public i18n: ReturnType<typeof useSettingsI18n>,
  ) {}

  handleActionPrivate = async () => {
    this.publishSpace.mutateAsync({
      spacePk: this.data.space.data.pk,
      visibility: { type: 'PRIVATE' },
    });
  };

  handleActionPublic = async () => {
    this.publishSpace.mutateAsync({
      spacePk: this.data.space.data.pk,
      visibility: { type: 'PUBLIC' },
    });
  };

  handleAnonymousParticipationChange = async (value: boolean) => {
    try {
      await this.updateAnonymousParticipation.mutateAsync({
        spacePk: this.spacePk,
        anonymousParticipation: value,
      });
      showSuccessToast(this.i18n.success_update);
    } catch (error) {
      logger.error('Failed to update anonymous participation', error);
      showErrorToast(this.i18n.error_update);
    }
  };

  handleChangeVisibilityChange = async (value: boolean) => {
    try {
      await this.updateChangeVisibility.mutateAsync({
        spacePk: this.spacePk,
        changeVisibility: value,
      });
      showSuccessToast(this.i18n.success_visibility_update);
    } catch (error) {
      logger.error('Failed to update change visibility', error);
      showErrorToast(this.i18n.error_visibility_update);
    }
  };

  get anonymousParticipation() {
    return this.data.space.data?.anonymous_participation ?? false;
  }

  // get changeVisibility() {
  //   return this.data.space.data?.change_visibility ?? false;
  // }
}

export function useSpaceSettingsController(spacePk: string) {
  const data = useSpaceSettingsData(spacePk);
  const state = useState(false);
  const updateAnonymousParticipation =
    useSpaceUpdateAnonymousParticipationMutation();
  const updateChangeVisibility = useSpaceUpdateChangeVisibilityMutation();
  const i18n = useSettingsI18n();
  const publishSpace = usePublishSpaceMutation();

  return new SpaceSettingsController(
    data,
    new State(state),
    publishSpace,
    updateAnonymousParticipation,
    updateChangeVisibility,
    spacePk,
    i18n,
  );
}
