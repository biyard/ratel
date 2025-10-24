import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import useFileSpace from '../../hooks/use-file-space';
import { Space } from '@/features/spaces/types/space';
import FileType from '../../types/file';

export class SpaceFileViewerController {
  constructor(
    public spacePk: string,
    public space: Space,
    public files: FileType[],
  ) {}
}

export function useSpaceFileViewerController(spacePk) {
  const { data: space } = useSpaceById(spacePk);
  const { data: file } = useFileSpace(spacePk);
  const files = file.files;

  return new SpaceFileViewerController(spacePk, space, files);
}
