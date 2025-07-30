'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { FileInfo } from '@/lib/api/models/feeds';
import React from 'react';

import { downloadPdfFromUrl } from '@/lib/pdf-utils';
import { checkString } from '@/lib/string-filter-utils';
import SpaceFile from '../../_components/space_file';

import FileUploaderMetadata from '@/components/file-uploader-metadata';
import { Upload } from 'lucide-react';
import {
  CircleClose,
  Excel,
  Jpg,
  MP4,
  Pdf,
  Png,
  Pptx,
  Word,
  Zip,
} from '@/components/icons';

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
        <div className="flex flex-row w-full justify-between items-start ">
          <div className="font-bold text-white text-[15px]/[20px]">
            Attached Files
          </div>

          {isEdit ? (
            <FileUploaderMetadata
              isImage={false}
              isMedia={true}
              onUploadSuccess={(file) => {
                onadd(file);
              }}
            >
              <div className="cursor-pointer flex flex-row w-fit gap-1 items-center bg-white rounded-[6px] px-[14px] py-[8px] hover:bg-neutral-300">
                <Upload className="w-5 h-5 stroke-neutral-500" />
                <div className="font-bold text-sm text-[#000203]">Upload</div>
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
    </BlackBox>
  );
}

function EditableFile({
  file,
  onclick,
}: {
  file: FileInfo;
  onclick: () => void;
}) {
  console.log('EditableFile:', file);
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
        ) : file.ext === 'MP4' ? (
          <MP4 />
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
        <CircleClose className="w-[18px] h-[18px]" />
      </div>
    </div>
  );
}
