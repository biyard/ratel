'use client';

import FileUploaderMetadata from '@/components/file-uploader-metadata';
import { FileExtension, toFileExtension } from '@/lib/api/models/feeds';
// import { checkString } from '@/lib/string-filter-utils';

import React from 'react';
import { ArrowRight, Upload } from 'lucide-react';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { CircleClose } from '@/components/icons';
import { useTranslations } from 'next-intl';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';
import { useDeliberationSpaceByIdContext } from '../providers.client';
import { File } from '@/lib/api/models/spaces/deliberation-spaces';

export default function SpaceElearning() {
  const t = useTranslations('DeliberationSpace');
  const { isEdit, deliberation, handleUpdateDeliberation } =
    useDeliberationSpaceByIdContext();
  const elearnings = deliberation.elearnings;

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
            {t('elearnings')}
          </div>

          {isEdit ? (
            <FileUploaderMetadata
              isImage={false}
              isMedia={true}
              onUploadSuccess={(file) => {
                const f: File = {
                  name: file.name ?? 'untitled',
                  size: file.size,
                  ext: toFileExtension(file.ext),
                  url: file.url,
                };

                handleUpdateDeliberation({
                  ...deliberation,
                  elearnings: [...deliberation.elearnings, { files: [f] }],
                });
              }}
            >
              <div className="cursor-pointer flex flex-row w-fit gap-1 items-center bg-white light:bg-card-bg border border-card-border hover:bg-white/80 light:hover:bg-card-bg/50 rounded-[6px] px-[14px] py-[8px]">
                <Upload className="w-5 h-5 stroke-neutral-600" />
                <div className="font-bold text-sm text-black">
                  {t('upload')}
                </div>
              </div>
            </FileUploaderMetadata>
          ) : (
            <></>
          )}
        </div>

        {isEdit ? (
          <div className="flex flex-col w-full gap-2.5">
            <div className="flex flex-col w-full gap-2.5">
              {elearnings
                ?.filter((file) => file.files.length != 0)
                .map((file, index) => (
                  <div
                    className="flex flex-col w-full"
                    key={file?.files[0].name ?? ''}
                  >
                    <EditableFile
                      file={file.files[0]}
                      onclick={() => {
                        const updated = deliberation.elearnings.filter(
                          (_, i) => i !== index,
                        );
                        handleUpdateDeliberation({
                          ...deliberation,
                          elearnings: updated,
                        });
                      }}
                    />

                    {index !== elearnings.length - 1 ? (
                      <div className="flex flex-row w-full h-0.25 bg-neutral-800 light:bg-[#e5e5e5]" />
                    ) : (
                      <></>
                    )}
                  </div>
                ))}
            </div>
          </div>
        ) : (
          <div className="flex flex-col w-full gap-2.5">
            {elearnings.map((file, index) => (
              <EBook
                file={file.files[0]}
                key={'file ' + index}
                onClick={() => handlePdfDownload(file.files[0])}
              />
            ))}
          </div>
        )}
      </div>
    </BorderSpaceCard>
  );
}

//FIXME: implement pdf reader
function EBook({ file, onClick }: { file: File; onClick: () => void }) {
  return (
    <div className="flex flex-row justify-between items-center pb-2.5 border-b border-b-neutral-800 light:border-b-[#e5e5e5]">
      <div className="flex flex-col gap-1">
        <div className="font-normal text-neutral-400 light:text-[#737373] text-sm">
          {file?.ext === FileExtension.MP4 || file?.ext === FileExtension.MOV
            ? 'eLearning'
            : 'eBook'}
        </div>
        <div className="font-bold text-text-primary text-lg">
          {file?.name.replace(/\.[^/.]+$/, '')}
        </div>
      </div>
      <ReadButton onClick={onClick} />
    </div>
  );
}

function ReadButton({ onClick }: { onClick: () => void }) {
  const t = useTranslations('DeliberationSpace');
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5  bg-white light:bg-card-bg border border-card-border hover:bg-white/80 light:hover:bg-card-bg/50 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">{t('read')}</div>
      <ArrowRight className="stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}

function EditableFile({ file, onclick }: { file: File; onclick: () => void }) {
  const t = useTranslations('DeliberationSpace');
  return (
    <div className="cursor-pointer flex flex-row justify-start items-center w-full py-5 gap-2 bg-transparent rounded-[8px] mt-[10px]">
      <div className="flex flex-col w-full justify-start items-start gap-1">
        <div className="font-normal text-sm text-time-text">
          {file.ext === FileExtension.MP4 || file.ext === FileExtension.MOV
            ? t('elearning')
            : t('ebook')}
        </div>
        <div className="font-bold text-lg text-neutral-300 light:text-text-primary">
          {file.name}
        </div>
      </div>
      <div className="w-fit h-fit cursor-pointer" onClick={onclick}>
        <CircleClose className="w-4.5 h-4.5" fill="white" />
      </div>
    </div>
  );
}
