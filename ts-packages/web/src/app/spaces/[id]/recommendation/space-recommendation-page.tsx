import { useParams } from 'react-router';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceRecommendationEditorPage } from '@/features/spaces/recommendations/pages/creator/space-recommendation-editor-page';
import { SpaceRecommendationViewerPage } from '@/features/spaces/recommendations/pages/viewer/space-recommendation-viewer-page';

export default function SpaceRecommendationPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const { data: space } = useSpaceById(spacePk);

  if (!space) {
    throw new Error('Space not found');
  }

  if (space.isAdmin()) {
    // Edit Mode
    return <SpaceRecommendationEditorPage spacePk={spacePk} />;
  }

  return <SpaceRecommendationViewerPage spacePk={spacePk} />;
}
