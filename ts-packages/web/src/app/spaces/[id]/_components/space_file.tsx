import { FileInfo } from '@/lib/api/models/feeds';
import React from 'react';
import Jpg from '@/assets/icons/files/jpg.svg';
import Png from '@/assets/icons/files/png.svg';
import Pdf from '@/assets/icons/files/pdf.svg';
import Zip from '@/assets/icons/files/zip.svg';
import Word from '@/assets/icons/files/docx.svg';
import Pptx from '@/assets/icons/files/pptx.svg';
import Excel from '@/assets/icons/files/xlsx.svg';
import Upload from '@/assets/icons/upload.svg';

export default function SpaceFile({
  file,
  onClick,
}: {
  file: FileInfo;
  onClick: () => void;
}) {
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-neutral-800 rounded-[8px]`}
      onClick={onClick}
    >
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
      <Upload width={16} height={16} />
    </div>
  );
}
