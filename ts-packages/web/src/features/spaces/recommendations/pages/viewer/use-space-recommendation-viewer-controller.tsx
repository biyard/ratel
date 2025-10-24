import { Space } from '@/features/spaces/types/space';
import useRecommendationSpace from '../../hooks/use-recommendation-space';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import FileType from '@/features/spaces/files/types/file';

export class SpaceRecommendationViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public files: FileType[],
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
