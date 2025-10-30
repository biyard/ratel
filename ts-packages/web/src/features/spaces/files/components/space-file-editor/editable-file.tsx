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
import FileModel, { FileExtension } from '../../types/file';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';

export interface EditableFileProps {
  file: FileModel;
  onclick: () => void;
}

export default function EditableFile({ file, onclick }: EditableFileProps) {
  const handleDownload = async (file: FileModel) => {
    await downloadPdfFromUrl({
      url: file.url ?? '',
      fileName: file.name,
    });
  };

  return (
    <div
      className="cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-[#262626] light:bg-card-bg border border-card-border rounded-[8px]"
      onClick={() => {
        handleDownload(file);
      }}
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
      <div
        className="w-fit h-fit cursor-pointer"
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          onclick();
        }}
      >
        <CircleClose className="w-[18px] h-[18px]" />
      </div>
    </div>
  );
}
