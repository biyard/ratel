import { FileInfo } from '@/lib/api/models/feeds';

import {
  Jpg,
  Png,
  Pdf,
  Zip,
  Word,
  Pptx,
  Excel,
  Upload,
} from '@/components/icons';
export function File({
  file,
  onClick,
}: {
  file: FileInfo;
  onClick?: () => void;
}) {
  const handleDownload = () => {
    if (onClick) {
      onClick();
      return;
    }

    if (!file.url) {
      return;
    }

    const link = document.createElement('a');
    link.href = file.url;
    link.download = file.name;
    link.target = '_blank';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };
  return (
    <div
      className={`cursor-pointer flex flex-row justify-start items-center w-full gap-2 p-4 bg-neutral-800 rounded-[8px]`}
      onClick={handleDownload}
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
