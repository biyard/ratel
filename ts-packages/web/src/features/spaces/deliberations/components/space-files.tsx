'use client';

import { FileExtension, toFileExtension } from '@/lib/api/models/feeds';

import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { checkString } from '@/lib/string-filter-utils';

import FileUploaderMetadata from '@/features/spaces/files/components/file-uploader-metadata';
import { Upload } from 'lucide-react';
import {
  CircleClose,
  Excel,
  Jpg,
  MOV,
  MP4,
  Pdf,
  Png,
  Pptx,
  Word,
  Zip,
} from '@/components/icons';
import { useTranslation } from 'react-i18next';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';
import { File } from '../utils/deliberation.spaces.v3';

export interface SpaceFilesProps {
  isEdit?: boolean;
  files: File[];
  onremove?: (index: number) => void;
  onadd?: (file: File) => void;
}

export default function SpaceFiles({
  isEdit = false,
  files,
  onremove = () => {},
  onadd = () => {},
}: SpaceFilesProps) {
  const { t } = useTranslation('DeliberationSpace');
  const handlePdfDownload = async (file: File) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };
  return (
    <BorderSpaceCard>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-row w-full justify-between items-start ">
          <div className="font-bold text-text-primary text-[15px]/[20px]">
            {t('attached_files')}
          </div>

          {isEdit ? (
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
          ) : (
            <></>
          )}
        </div>

        {isEdit ? (
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
        ) : (
          <div className="grid grid-cols-1 max-tablet:grid-cols-1 gap-2.5">
            {files
              ?.filter((file) => !checkString(file.name))
              .map((file, index) => (
                <SpaceFile
                  file={file}
                  key={'file ' + index}
                  onClick={() => handlePdfDownload(file)}
                />
              ))}
          </div>
        )}
      </div>
    </BorderSpaceCard>
  );
}

function SpaceFile({ file, onClick }: { file: File; onClick: () => void }) {
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]`}
      onClick={onClick}
    >
      <div className="[&>svg]:size-9">
        {file.ext === FileExtension.JPG ? (
          <Jpg />
        ) : file.ext === FileExtension.PNG ? (
          <Png />
        ) : file.ext === FileExtension.PDF ? (
          <Pdf />
        ) : file.ext === FileExtension.ZIP ? (
          <Zip />
        ) : file.ext === FileExtension.WORD ? (
          <Word />
        ) : file.ext === FileExtension.PPTX ? (
          <Pptx />
        ) : file.ext === FileExtension.MP4 ? (
          <MP4 />
        ) : file.ext === FileExtension.MOV ? (
          <MOV />
        ) : (
          <Excel />
        )}
      </div>
      <div className="flex flex-col w-full justify-start items-start gap-2">
        <div className="font-semibold text-xs/[18px] text-text-primary">
          {file.name}
        </div>
        <div className="font-normal text-[10px]/[16px] text-time-text">
          {file.size}
        </div>
      </div>
      <Upload width={16} height={16} />
    </div>
  );
}

function EditableFile({ file, onclick }: { file: File; onclick: () => void }) {
  return (
    <div className="cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]">
      <div className="[&>svg]:size-9">
        {file.ext === FileExtension.JPG ? (
          <Jpg />
        ) : file.ext === FileExtension.PNG ? (
          <Png />
        ) : file.ext === FileExtension.PDF ? (
          <Pdf />
        ) : file.ext === FileExtension.ZIP ? (
          <Zip />
        ) : file.ext === FileExtension.WORD ? (
          <Word />
        ) : file.ext === FileExtension.PPTX ? (
          <Pptx />
        ) : file.ext === FileExtension.MP4 ? (
          <MP4 />
        ) : file.ext === FileExtension.MOV ? (
          <MOV />
        ) : (
          <Excel />
        )}
      </div>
      <div className="flex flex-col w-full justify-start items-start gap-2">
        <div className="font-semibold text-xs/[18px] text-text-primary">
          {file.name}
        </div>
        <div className="font-normal text-[10px]/[16px] text-time-text">
          {file.size}
        </div>
      </div>
      <div className="w-fit h-fit cursor-pointer" onClick={onclick}>
        <CircleClose className="w-[18px] h-[18px]" />
      </div>
    </div>
  );
}
