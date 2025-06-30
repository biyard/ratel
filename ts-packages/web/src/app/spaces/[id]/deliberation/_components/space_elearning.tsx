'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import FileUploaderMetadata from '@/components/file-uploader-metadata';
import { FileInfo } from '@/lib/api/models/feeds';
import { checkString } from '@/lib/string-filter-utils';

import Jpg from '@/assets/icons/files/jpg.svg';
import Png from '@/assets/icons/files/png.svg';
import Pdf from '@/assets/icons/files/pdf.svg';
import Zip from '@/assets/icons/files/zip.svg';
import Word from '@/assets/icons/files/docx.svg';
import Pptx from '@/assets/icons/files/pptx.svg';
import Excel from '@/assets/icons/files/xlsx.svg';
import Clear from '@/assets/icons/clear.svg';
import React from 'react';
import { ArrowRight } from 'lucide-react';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { ElearningCreateRequest } from '@/lib/api/models/elearning';

export interface SpaceElearningProps {
  isEdit?: boolean;
  elearnings: ElearningCreateRequest[];
  onremove?: (index: number) => void;
  onadd?: (index: FileInfo) => void;
}

export default function SpaceElearning({
  isEdit = false,
  elearnings,
  onremove = () => {},
  onadd = () => {},
}: SpaceElearningProps) {
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
          e-Learnings
        </div>

        {isEdit ? (
          <div className="flex flex-col w-full gap-[10px]">
            <AddFile onadd={onadd} />
            <div className="flex flex-col w-full gap-2.5">
              {elearnings
                ?.filter((file) => !checkString(file.files[0].name))
                .map((file, index) => (
                  <EditableFile
                    key={index}
                    file={file.files[0]}
                    onclick={() => {
                      onremove(index);
                    }}
                  />
                ))}
            </div>
          </div>
        ) : (
          <div className="flex flex-col w-full gap-[10px]">
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
    <div className="flex flex-row justify-between items-center pb-[10px] border-b border-b-neutral-800">
      <div className="flex flex-col gap-1">
        <div className="font-normal text-neutral-400 text-sm">eBook</div>
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

function AddFile({ onadd }: { onadd: (file: FileInfo) => void }) {
  return (
    <FileUploaderMetadata
      onUploadSuccess={(file) => {
        onadd(file);
      }}
      isImage={false}
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
