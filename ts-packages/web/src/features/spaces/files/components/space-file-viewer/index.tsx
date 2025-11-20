import Card from '@/components/card';
import { checkString } from '@/lib/string-filter-utils';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import SpaceFile from './space-file';
import FileModel from '../../types/file';
import { useEffect, useState } from 'react';

const IMAGE_EXTS = ['jpg', 'jpeg', 'png', 'gif', 'webp'];
const VIDEO_EXTS = ['mp4', 'mov', 'webm', 'mkv'];
const PDF_EXTS = ['pdf'];

export interface SpaceFilesProps {
  files: FileModel[];
}

export default function SpaceFileViewer({ files }: SpaceFilesProps) {
  const handlePdfDownload = async (file: FileModel) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };

  const isImage = (ext?: string) =>
    !!ext &&
    ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(ext.toLowerCase());

  const isVideo = (ext?: string) =>
    !!ext && ['mp4', 'mov', 'avi', 'mkv', 'webm'].includes(ext.toLowerCase());

  const isPdf = (ext?: string) => !!ext && ['pdf'].includes(ext.toLowerCase());

  const imageFiles = files.filter((f) => {
    const name = f.name.toLowerCase();
    return isImage(f.ext) || IMAGE_EXTS.some((ext) => name.includes(`.${ext}`));
  });

  const videoFiles = files.filter((f) => {
    const name = f.name.toLowerCase();
    console.log('file: ', f.ext, f.name);
    return isVideo(f.ext) || VIDEO_EXTS.some((ext) => name.includes(`.${ext}`));
  });

  const pdfFiles = files.filter((f) => {
    const name = f.name.toLowerCase();
    return isPdf(f.ext) || PDF_EXTS.some((ext) => name.includes(`.${ext}`));
  });

  const [previewUrl, setPreviewUrl] = useState<string | null>(
    () => pdfFiles?.[0]?.url ?? null,
  );

  useEffect(() => {
    if (!files || files.length === 0) {
      setPreviewUrl(null);
      return;
    }

    setPreviewUrl((prev) => {
      if (prev && pdfFiles.some((f) => f.url === prev)) {
        return prev;
      }
      return pdfFiles[0]?.url ?? null;
    });
  }, [pdfFiles]);

  console.log('video files: ', videoFiles);

  return (
    <Card>
      <div className="flex flex-col w-full gap-5">
        <div className="grid grid-cols-1 max-tablet:grid-cols-1 gap-2.5">
          {files
            ?.filter((file) => !checkString(file.name))
            .map((file, index) => (
              <SpaceFile
                file={file}
                key={'file ' + index}
                onclick={() => {
                  if (isPdf(file.ext)) {
                    handlePdfDownload(file);
                    setPreviewUrl(file.url);
                  }
                }}
              />
            ))}
        </div>

        {previewUrl && (
          <div className="mt-4">
            <iframe
              src={`${previewUrl}`}
              className="w-full h-[600px] rounded-lg"
              title="PDF preview"
            />
          </div>
        )}

        {(videoFiles.length > 0 || imageFiles.length > 0) && (
          <div className="flex flex-col gap-6 mt-4 border-t border-neutral-700 pt-4">
            {videoFiles.length > 0 && (
              <div className="flex flex-col gap-3">
                {videoFiles.map((file, i) => (
                  <video
                    key={'video ' + i}
                    src={file.url ?? ''}
                    controls
                    className="w-full rounded-lg border border-border max-h-[500px]"
                  />
                ))}
              </div>
            )}

            {imageFiles.length > 0 && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {imageFiles.map((file, i) => (
                  <img
                    key={'img ' + i}
                    src={file.url ?? ''}
                    alt={file.name}
                    className="w-full object-contain rounded-lg border border-border max-h-[500px] bg-black"
                  />
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </Card>
  );
}
