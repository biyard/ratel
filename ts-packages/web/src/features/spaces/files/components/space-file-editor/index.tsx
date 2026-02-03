import Card from '@/components/card';
import FileUploaderMetadata, {
  FileUploaderHandle,
} from '@/features/spaces/files/components/file-uploader-metadata';
import { checkString } from '@/lib/string-filter-utils';
import EditableFile from './editable-file';
import { useTranslation } from 'react-i18next';
import FileModel, { toFileExtension } from '../../types/file';
import { useCallback, useRef, useState } from 'react';

const IMAGE_EXTS = ['jpg', 'jpeg', 'png', 'gif', 'webp'];
const VIDEO_EXTS = ['mp4', 'mov', 'webm', 'mkv'];

export interface SpaceFilesEditorProps {
  files: FileModel[];
  onremove?: (fileId: string) => void;
  onadd?: (file: FileModel) => void;
}

export default function SpaceFileEditors({
  files,
  onremove = () => {},
  onadd = () => {},
}: SpaceFilesEditorProps) {
  const { t } = useTranslation('SpaceFile');
  const uploaderRef = useRef<FileUploaderHandle | null>(null);
  const [dragActive, setDragActive] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const onDrop = useCallback(async (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    const file = e.dataTransfer.files?.[0];
    if (!file) return;
    await uploaderRef.current?.uploadFile(file);
  }, []);

  const onDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(true);
  }, []);

  const onDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
  }, []);

  const isImage = (ext?: string) =>
    !!ext &&
    ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(ext.toLowerCase());

  const isVideo = (ext?: string) =>
    !!ext && ['mp4', 'mov', 'avi', 'mkv', 'webm'].includes(ext.toLowerCase());

  const imageFiles = files.filter((f) => {
    const name = f.name.toLowerCase();
    return isImage(f.ext) || IMAGE_EXTS.some((ext) => name.includes(`.${ext}`));
  });

  const videoFiles = files.filter((f) => {
    const name = f.name.toLowerCase();
    return isVideo(f.ext) || VIDEO_EXTS.some((ext) => name.includes(`.${ext}`));
  });
  return (
    <Card>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-row w-full justify-between items-start ">
          <div className="font-bold text-text-primary text-[15px]/[20px]">
            {t('attached_files')}
          </div>
        </div>

        <FileUploaderMetadata
          ref={uploaderRef}
          maxSizeMB={50}
          disabled={isLoading}
          onUploadingChange={setIsLoading}
          onUploadSuccess={(uploaded) => {
            const f: FileModel = {
              id: uploaded.id,
              name: uploaded.name ?? 'untitled',
              size: uploaded.size,
              ext: toFileExtension(uploaded.ext),
              url: uploaded.url,
            };
            onadd(f);
          }}
        >
          <div
            onClick={() => uploaderRef.current?.openPicker()}
            onDrop={onDrop}
            onDragOver={onDragOver}
            onDragLeave={onDragLeave}
            className={[
              'relative w-full min-h-[140px]',
              'rounded-xl border-2 border-dashed',
              dragActive
                ? 'border-blue-500 bg-blue-50 ring-2 ring-blue-200'
                : 'border-neutral-300 hover:border-neutral-400',
              'transition-colors duration-150 ease-in-out',
              isLoading ? 'pointer-events-none opacity-60' : 'cursor-pointer',
              'flex items-center justify-center',
            ].join(' ')}
          >
            <div className="flex flex-col items-center gap-2">
              <div className="w-10 h-10 rounded-full border border-neutral-400 flex items-center justify-center text-neutral-600 text-2xl leading-none">
                +
              </div>
              <div className="text-sm text-neutral-600 font-medium">
                {isLoading ? t('uploading') : t('upload')}
              </div>
              <div className="text-xs text-neutral-500">
                {t(
                  'drag_or_click_to_upload',
                  'Drag & drop a file here, or click to upload (max 50MB)',
                )}
              </div>
            </div>
          </div>
        </FileUploaderMetadata>

        <div className="flex flex-col w-full gap-[10px]">
          {files
            ?.filter((file) => !checkString(file.name))
            .map((file, index) => (
              <EditableFile
                key={file.id || index}
                file={file}
                onclick={() => onremove(file.id)}
              />
            ))}
        </div>

        {(videoFiles.length > 0 || imageFiles.length > 0) && (
          <div className="flex flex-col gap-6 mt-4 pt-4">
            {videoFiles.length > 0 && (
              <div className="flex flex-col gap-3">
                {videoFiles.map((file, i) => (
                  <video
                    key={'video-' + i}
                    src={file.url ?? ''}
                    controls
                    className="w-full rounded-lg max-h-[480px]"
                  />
                ))}
              </div>
            )}

            {imageFiles.length > 0 && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {imageFiles.map((file, i) => (
                  <img
                    key={'image-' + i}
                    src={file.url ?? ''}
                    alt={file.name}
                    className="w-full rounded-lg object-contain max-h-[480px]"
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
