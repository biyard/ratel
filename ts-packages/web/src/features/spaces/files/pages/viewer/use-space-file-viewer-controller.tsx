import useSpaceById from '@/hooks/use-space-by-id';
import useFileSpace from '../../hooks/use-file-space';
import { Space } from '@/lib/api/models/spaces';
import { File } from '@/lib/api/models/feeds';

export class SpaceFileViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public files: File[],
  ) {}
}

export function useSpaceFileViewerController(spacePk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: file } = useFileSpace(spacePk);
  const files = file.files;

  return new SpaceFileViewerController(spacePk, space, files);
}
