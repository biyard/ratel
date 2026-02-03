import Card from '@/components/card';
import { Upload } from '@/components/icons';
import { checkString } from '@/lib/string-filter-utils';
import FileModel, { toFileExtension } from '../../files/types/file';
import MediaUploaderMetadata from '../../files/components/media-uploader-metadata';
import EditableFile from '../../files/components/space-file-editor/editable-file';
import { useState } from 'react';
import { TFunction } from 'i18next';

export interface MediaFileEditorProps {
  t: TFunction<'Space'>;
  files: FileModel[];
  onremove?: (index: number) => void;
  onadd?: (file: FileModel) => void;
}

export default function MediaFileEditor({
  t,
  files,
  onremove = () => {},
  onadd = () => {},
}: MediaFileEditorProps) {
  const [isLoading, setIsLoading] = useState(false);

  return (
    <Card>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-col w-full gap-2">
          <div className="flex flex-row w-full justify-between items-start">
            <MediaUploaderMetadata
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
              <button
                type="button"
                disabled={isLoading}
                className="cursor-pointer flex flex-row w-fit gap-2 items-center bg-white border border-button-border rounded-[6px] px-[14px] py-[8px] hover:bg-neutral-300 disabled:opacity-60 disabled:cursor-not-allowed"
              >
                {isLoading ? (
                  <>
                    <svg
                      className="animate-spin h-5 w-5"
                      viewBox="0 0 24 24"
                      fill="none"
                    >
                      <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                      />
                      <path
                        className="opacity-75"
                        d="M4 12a8 8 0 018-8"
                        stroke="currentColor"
                        strokeWidth="4"
                        strokeLinecap="round"
                      />
                    </svg>
                    <div className="font-bold text-sm text-[#000203]">
                      {t('uploading')}
                    </div>
                  </>
                ) : (
                  <>
                    <Upload className="w-5 h-5 [&>path]:stroke-neutral-600" />
                    <div className="font-bold text-sm text-[#000203]">
                      {t('upload_media')}
                    </div>
                  </>
                )}
              </button>
            </MediaUploaderMetadata>
          </div>

          <p className="text-xs text-neutral-500">
            {t('upload_file_size_limit')}
          </p>
        </div>

        <div className="flex flex-col w-full gap-[10px]">
          <div className="flex flex-col w-full gap-2.5">
            {files
              ?.filter((file) => !checkString(file.name))
              .map((file, index) => (
                <EditableFile
                  key={index}
                  file={file}
                  onclick={() => {
                    onremove(index);
                  }}
                />
              ))}
          </div>
        </div>
      </div>
    </Card>
  );
}
