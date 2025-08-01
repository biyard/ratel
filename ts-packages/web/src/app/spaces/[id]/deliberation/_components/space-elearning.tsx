'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import FileUploaderMetadata from '@/components/file-uploader-metadata';
import { FileInfo } from '@/lib/api/models/feeds';
import { checkString } from '@/lib/string-filter-utils';

import React from 'react';
import { ArrowRight, Upload } from 'lucide-react';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { CircleClose } from '@/components/icons';
import { useDeliberationSpaceContext } from '../provider.client';

export default function SpaceElearning() {
  const { isEdit, deliberation, handleUpdateDeliberation } =
    useDeliberationSpaceContext();
  const elearnings = deliberation.elearnings;

  const handlePdfDownload = async (file: FileInfo) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };

  return (
    <BlackBox>
      <div className="flex flex-col w-full gap-5">
        <div className="flex flex-row w-full justify-between items-start ">
          <div className="font-bold text-white text-[15px]/[20px]">
            e-Learnings
          </div>

          {isEdit ? (
            <FileUploaderMetadata
              isImage={false}
              isMedia={true}
              onUploadSuccess={(file) => {
                handleUpdateDeliberation({
                  ...deliberation,
                  elearnings: [...deliberation.elearnings, { files: [file] }],
                });
              }}
            >
              <div className="cursor-pointer flex flex-row w-fit gap-1 items-center bg-white rounded-[6px] px-[14px] py-[8px] hover:bg-neutral-300">
                <Upload className="w-5 h-5 stroke-neutral-500" />
                <div className="font-bold text-sm text-black">Upload</div>
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
                ?.filter((file) => !checkString(file.files[0].name))
                .map((file, index) => (
                  <div
                    className="flex flex-col w-full"
                    key={file.files[0].name}
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
                      <div className="flex flex-row w-full h-0.25 bg-neutral-800" />
                    ) : (
                      <></>
                    )}
                  </div>
                ))}
            </div>
          </div>
        ) : (
          <div className="flex flex-col w-full gap-2.5">
            {elearnings
              ?.filter((file) => !checkString(file.files[0].name))
              .map((file, index) => (
                <EBook
                  file={file.files[0]}
                  key={'file ' + index}
                  onClick={() => handlePdfDownload(file.files[0])}
                />
              ))}
          </div>
        )}
      </div>
    </BlackBox>
  );
}

//FIXME: implement pdf reader
function EBook({ file, onClick }: { file: FileInfo; onClick: () => void }) {
  return (
    <div className="flex flex-row justify-between items-center pb-2.5 border-b border-b-neutral-800">
      <div className="flex flex-col gap-1">
        <div className="font-normal text-neutral-400 text-sm">
          {file.ext === 'MP4' || file.ext === 'MOV' ? 'eLearning' : 'eBook'}
        </div>
        <div className="font-bold text-white text-lg">
          {file.name.replace(/\.[^/.]+$/, '')}
        </div>
      </div>
      <ReadButton onClick={onClick} />
    </div>
  );
}

function ReadButton({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row items-center w-fit h-fit px-5 py-2.5 gap-2.5 bg-white hover:bg-neutral-300 rounded-lg"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-sm">Read</div>
      <ArrowRight className="stroke-black stroke-3 w-[15px] h-[15px]" />
    </div>
  );
}

function EditableFile({
  file,
  onclick,
}: {
  file: FileInfo;
  onclick: () => void;
}) {
  return (
    <div className="cursor-pointer flex flex-row justify-start items-center w-full py-5 gap-2 bg-transparent rounded-[8px] mt-[10px]">
      <div className="flex flex-col w-full justify-start items-start gap-1">
        <div className="font-normal text-sm text-neutral-400">
          {file.ext === 'MP4' || file.ext === 'MOV' ? 'eLearning' : 'eBook'}
        </div>
        <div className="font-bold text-lg text-neutral-300">{file.name}</div>
      </div>
      <div className="w-fit h-fit cursor-pointer" onClick={onclick}>
        <CircleClose className="w-4.5 h-4.5" fill="white" />
      </div>
    </div>
  );
}
