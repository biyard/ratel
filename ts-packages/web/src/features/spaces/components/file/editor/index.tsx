import { File, toFileExtension } from '@/lib/api/models/feeds';
import Card from '@/components/card';
import FileUploaderMetadata from '@/components/file-uploader-metadata';
import { Upload } from '@/components/icons';
import { checkString } from '@/lib/string-filter-utils';
import EditableFile from './editable-file';
import { useTranslation } from 'react-i18next';

export interface SpaceFilesEditorProps {
  files: File[];
  onremove?: (index: number) => void;
  onadd?: (file: File) => void;
}

export default function SpaceFileEditors({
  files,
  onremove = () => {},
  onadd = () => {},
}: SpaceFilesEditorProps) {
  const { t } = useTranslation('SpaceFile');
  return (
    <Card>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-row w-full justify-between items-start ">
          <div className="font-bold text-text-primary text-[15px]/[20px]">
            {t('attached_files')}
          </div>

          <FileUploaderMetadata
            isImage={false}
            isMedia={true}
            onUploadSuccess={(uploaded) => {
              const f: File = {
                name: uploaded.name ?? 'untitled',
                size: uploaded.size,
                ext: toFileExtension(uploaded.ext),
                url: uploaded.url,
              };

              onadd(f);
            }}
          >
            <div className="cursor-pointer flex flex-row w-fit gap-1 items-center bg-white border border-button-border rounded-[6px] px-[14px] py-[8px] hover:bg-neutral-300">
              <Upload className="w-5 h-5 [&>path]:stroke-neutral-600" />
              <div className="font-bold text-sm text-[#000203]">
                {t('upload')}
              </div>
            </div>
          </FileUploaderMetadata>
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
