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
import FileType, { FileExtension } from '../../types/file';

export interface EditableFileProps {
  file: FileType;
  onclick: () => void;
}

export default function EditableFile({ file, onclick }: EditableFileProps) {
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
