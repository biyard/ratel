import { useParams } from 'react-router';
import { SpaceHeaderProvider } from '../../_components/header/provider';
import SpaceHeader from '../../_components/header';
import { useDeliberationSpaceController } from './use-deliberation-space-controller';
import { DeliberationTab } from './types';
import ThreadPage from '@/features/spaces/deliberations/components/tab/thread';
import DeliberationPage from '@/features/spaces/deliberations/components/tab/deliberation';
import { DeliberationSurveyPage } from '@/features/spaces/deliberations/components/tab/poll';
import FinalConsensusPage from '@/features/spaces/deliberations/components/tab/recommendation';
import DeliberationAnalyzePage from '@/features/spaces/deliberations/components/tab/analyze';
import SpaceSideMenu from '@/features/spaces/deliberations/components/space_side_menu';

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
              <ThreadPage {...ctrl.threadProps} />
            ) : ctrl.selectedType == DeliberationTab.DELIBERATION ? (
              <DeliberationPage {...ctrl.deliberationProps} />
            ) : ctrl.selectedType == DeliberationTab.POLL ? (
              <DeliberationSurveyPage {...ctrl.deliberationSurveyProps} />
            ) : ctrl.selectedType == DeliberationTab.RECOMMANDATION ? (
              <FinalConsensusPage {...ctrl.finalConsensusProps} />
            ) : (
              <DeliberationAnalyzePage {...ctrl.deliberationAnalyzeProps} />
            )}

            <SpaceSideMenu {...ctrl.spaceSidemenuProps} />
          </div>
        </div>
      </div>
    </div>
  );
}
