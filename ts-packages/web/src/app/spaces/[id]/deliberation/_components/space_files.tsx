'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { FileInfo } from '@/lib/api/models/feeds';
import React from 'react';

import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { checkString } from '@/lib/string-filter-utils';
import SpaceFile from '../../_components/space_file';

import Jpg from '@/assets/icons/files/jpg.svg';
import Png from '@/assets/icons/files/png.svg';
import Pdf from '@/assets/icons/files/pdf.svg';
import Zip from '@/assets/icons/files/zip.svg';
import Word from '@/assets/icons/files/docx.svg';
import Pptx from '@/assets/icons/files/pptx.svg';
import Excel from '@/assets/icons/files/xlsx.svg';
import Clear from '@/assets/icons/clear.svg';
import FileUploaderMetadata from '@/components/file-uploader-metadata';

export interface SpaceFilesProps {
  isEdit?: boolean;
  files: FileInfo[];
  onremove?: (index: number) => void;
  onadd?: (index: FileInfo) => void;
}

export default function SpaceFiles({
  isEdit = false,
  files,
  onremove = () => {},
  onadd = () => {},
}: SpaceFilesProps) {
  const handlePdfDownload = async (file: FileInfo) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };
  return (
    <BlackBox>
      <div className="flex flex-col w-full gap-5">
        <div className="font-bold text-white text-[15px]/[20px]">
          Attached Files
        </div>

        {isEdit ? (
          <div className="flex flex-col w-full gap-[10px]">
            <AddImage onadd={onadd} />
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
    </BlackBox>
  );
}

function AddImage({ onadd }: { onadd: (file: FileInfo) => void }) {
  return (
    <FileUploaderMetadata
      isImage={false}
      onUploadSuccess={(file) => {
        onadd(file);
      }}
    >
      <div className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] rounded-[8px] bg-neutral-700 hover:bg-neutral-600 font-medium text-white text-sm">
        Add File
      </div>
    </FileUploaderMetadata>
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
    <div className="cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-neutral-800 rounded-[8px]">
      <div className="[&>svg]:size-9">
        {file.ext === 'JPG' ? (
          <Jpg />
        ) : file.ext === 'PNG' ? (
          <Png />
        ) : file.ext === 'PDF' ? (
          <Pdf />
        ) : file.ext === 'ZIP' ? (
          <Zip />
        ) : file.ext === 'WORD' ? (
          <Word />
        ) : file.ext === 'PPTX' ? (
          <Pptx />
        ) : (
          <Excel />
        )}
      </div>
      <div className="flex flex-col w-full justify-start items-start gap-2">
        <div className="font-semibold text-xs/[18px] text-neutral-400">
          {file.name}
        </div>
        <div className="font-normal text-[10px]/[16px] text-[#6d6d6d]">
          {file.size}
        </div>
      </div>
      <div className="w-fit h-fit cursor-pointer" onClick={onclick}>
        <Clear
          className="w-[24px] h-[24px] [&>path]:stroke-neutral-500"
          fill="white"
        />
      </div>
    </div>
  );
}
