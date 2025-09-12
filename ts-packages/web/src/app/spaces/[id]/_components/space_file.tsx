import { FileInfo } from '@/lib/api/models/feeds';
import React from 'react';
import {
  Excel,
  Jpg,
  MP4,
  Pdf,
  Png,
  Pptx,
  Word,
  Zip,
  Upload,
  MOV,
} from '@/components/icons';

export default function SpaceFile({
  file,
  onClick,
}: {
  file: FileInfo;
  onClick: () => void;
}) {
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]`}
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
        ) : file.ext === 'MP4' ? (
          <MP4 />
        ) : file.ext === 'MOV' ? (
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
