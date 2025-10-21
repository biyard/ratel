import useSpaceById from '@/hooks/use-space-by-id';
import { Space } from '@/lib/api/models/spaces';
import { File } from '@/lib/api/models/feeds';
import useRecommendationSpace from '../../hooks/use-recommendation-space';

export class SpaceRecommendationViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public files: File[],
    public htmlContents: string,
  ) {}
}

export function useSpaceRecommendationViewerController(spacePk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: recommendation } = useRecommendationSpace(spacePk);
  const files = recommendation.files;
  const htmlContents = recommendation.html_contents;

  return new SpaceRecommendationViewerController(
    spacePk,
    space,
    files,
    htmlContents,
  );
}
