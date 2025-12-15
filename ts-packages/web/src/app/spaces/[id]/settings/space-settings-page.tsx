import { useParams } from 'react-router';
import { useSpaceSettingsController } from './use-space-settings-controller';
import { Checkbox } from '@/components/checkbox/checkbox';
import { useSettingsI18n } from './i18n';
import SwitchButton from '@/components/switch-button';
import { SpacePublishState } from '@/features/spaces/types/space-common';

export function SpaceSettingsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();

  const ctrl = useSpaceSettingsController(spacePk);
  const i18n = useSettingsI18n();

  const value = ctrl.data.space.data.isPublic;

  if (ctrl.data.space.isLoading) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="text-text-primary">{i18n.loading}</div>
      </div>
    );
  }

  if (ctrl.data.space.isError) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="text-red-500">{i18n.error}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6 p-6">
      <div className="text-2xl font-bold text-text-primary">{i18n.title}</div>

      {ctrl.data.space.data.publishState === SpacePublishState.Draft && (
        <div className="flex flex-col gap-4">
          <div className="text-lg font-semibold text-text-primary">
            {i18n.participation_title}
          </div>

          <Checkbox
            id="anonymous-participation"
            value={ctrl.anonymousParticipation}
            onChange={ctrl.handleAnonymousParticipationChange}
          >
            <span className="font-medium text-text-primary">
              {i18n.anonymous_participation_label}
            </span>
            {/* <Tooltip>
            <TooltipTrigger asChild>
              <Info />
            </TooltipTrigger>
            <TooltipContent>
              <p>Add to library</p>
            </TooltipContent>
          </Tooltip> */}
          </Checkbox>
        </div>
      )}

      {ctrl.data.space.data.publishState === SpacePublishState.Published && (
        <div className="flex flex-col gap-4">
          <div className="text-lg font-semibold text-text-primary">
            {i18n.public_setting}
          </div>

          <div className="flex flex-row w-fit gap-4 items-center">
            <span className="text-[15px]/[24px] font-normal text-text-primary">
              {value ? i18n.public : i18n.private}
            </span>
            <SwitchButton
              value={value}
              onChange={() => {
                if (value) {
                  ctrl.handleActionPrivate();
                } else {
                  ctrl.handleActionPublic();
                }
              }}
              color="bg-[#fcb300]"
            />
          </div>
        </div>
      )}
    </div>
  );
}
