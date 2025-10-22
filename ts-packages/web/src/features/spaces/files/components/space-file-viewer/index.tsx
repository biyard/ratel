import { File } from '@/lib/api/models/feeds';
import Card from '@/components/card';
import { checkString } from '@/lib/string-filter-utils';

import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import SpaceFile from './space-file';

export interface SpaceFilesProps {
  files: File[];
}

export default function SpaceFileViewer({ files }: SpaceFilesProps) {
  const handlePdfDownload = async (file: File) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };

  return (
    <Card>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-col w-full gap-[10px]">
          <div className="grid grid-cols-1 max-tablet:grid-cols-1 gap-2.5">
            {files
              ?.filter((file) => !checkString(file.name))
              .map((file, index) => (
                <SpaceFile
                  file={file}
                  key={'file ' + index}
                  onclick={() => handlePdfDownload(file)}
                />
              ))}
          </div>
        </div>
      </div>
    </Card>
  );
}
