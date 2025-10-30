import {
  Excel,
  Jpg,
  MOV,
  MP4,
  Pdf,
  Png,
  Pptx,
  Upload,
  Word,
  Zip,
} from '@/components/icons';
import FileModel, { FileExtension } from '@/features/spaces/files/types/file';

export interface SpaceFileProps {
  file: FileModel;
  onclick: () => void;
}

export default function SpaceFile({ file, onclick }: SpaceFileProps) {
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]`}
      onClick={onclick}
    >
      <div className="[&>svg]:size-9">
        {file.ext.toLowerCase() === FileExtension.JPG.toLowerCase() ? (
          <Jpg />
        ) : file.ext.toLowerCase() === FileExtension.PNG.toLowerCase() ? (
          <Png />
        ) : file.ext.toLowerCase() === FileExtension.PDF.toLowerCase() ? (
          <Pdf />
        ) : file.ext.toLowerCase() === FileExtension.ZIP.toLowerCase() ? (
          <Zip />
        ) : file.ext.toLowerCase() === FileExtension.WORD.toLowerCase() ? (
          <Word />
        ) : file.ext.toLowerCase() === FileExtension.PPTX.toLowerCase() ? (
          <Pptx />
        ) : file.ext.toLowerCase() === FileExtension.MP4.toLowerCase() ? (
          <MP4 />
        ) : file.ext.toLowerCase() === FileExtension.MOV.toLowerCase() ? (
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
