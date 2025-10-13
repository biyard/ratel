import { useParams } from 'react-router';
import { SpaceHeaderProvider } from '../../_components/header/provider';
import SpaceHeader from '../../_components/header';
import { useDeliberationSpaceController } from './use-deliberation-space-controller';
import { DeliberationTab, DeliberationTabType } from './types';
import SpaceSideMenu from './components/space_side_menu';

export default function DeliberationSpacePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useDeliberationSpaceController(spacePk);
  const hasEditPermission = true;

  return (
    <div className="flex flex-col w-full gap-6">
      <SpaceHeaderProvider
        post={ctrl.post.post}
        space={ctrl.space}
        hasEditPermission={hasEditPermission}
        onSave={ctrl.onSave}
      >
        <SpaceHeader />
      </SpaceHeaderProvider>

      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {ctrl.selectedType == DeliberationTab.SUMMARY ? (
              <div className="text-white">thread</div>
            ) : ctrl.selectedType == DeliberationTab.DELIBERATION ? (
              <div className="text-white">deliberation</div>
            ) : ctrl.selectedType == DeliberationTab.POLL ? (
              <div className="text-white">poll</div>
            ) : ctrl.selectedType == DeliberationTab.RECOMMANDATION ? (
              <div className="text-white">final consensus</div>
            ) : (
              <div className="text-white">analyze</div>
            )}

            <SpaceSideMenu
              space={ctrl.space}
              deliberation={ctrl.deliberation}
              selectedType={ctrl.selectedType}
              handleUpdateSelectedType={ctrl.setSelectedType}
              startedAt={ctrl.startedAt}
              endedAt={ctrl.endedAt}
              handleUpdateStartDate={ctrl.setStartDate}
              handleUpdateEndDate={ctrl.setEndDate}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
