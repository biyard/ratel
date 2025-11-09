import { useParams } from 'react-router';
import { useSpaceSettingsController } from './use-space-settings-controller';
import { Checkbox } from '@/components/checkbox/checkbox';
import { useSettingsI18n } from './i18n';

export function SpaceSettingsPage() {
  const { spacePk } = useParams<{ spacePk: string }>();

  const ctrl = useSpaceSettingsController(spacePk);
  const i18n = useSettingsI18n();

  if (ctrl.data.space.isLoading) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="text-white">{i18n.loading}</div>
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
      <div className="text-2xl font-bold text-white">{i18n.title}</div>

      <div className="flex flex-col gap-4">
        <div className="text-lg font-semibold text-white">
          {i18n.participation_title}
        </div>

        <Checkbox
          id="anonymous-participation"
          value={ctrl.anonymousParticipation}
          onChange={ctrl.handleAnonymousParticipationChange}
        >
          <span className="font-medium text-white">
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
    </div>
  );
}
