import { File, FileExtension } from '@/lib/api/models/feeds';
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

export interface SpaceFileProps {
  file: File;
  onclick: () => void;
}

export default function SpaceFile({ file, onclick }: SpaceFileProps) {
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]`}
      onClick={onclick}
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
