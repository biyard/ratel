'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import { Button } from '@/components/ui/button';
import { useState } from 'react';
import FileUploaderMetadata from '@/features/spaces/files/components/file-uploader-metadata';

import { UploadFile } from '@/assets/icons/file';
import { FileInfo } from '@/lib/api/models/feeds';
import { Input } from '@/components/ui/input';

const openCreateArtworkModal = (
  popup: ReturnType<typeof usePopup>,
  handleCreate: (
    title: string,
    description: string | null,
    file: FileInfo,
  ) => void,
) => {
  popup
    .open(
      <CreateArtworkModal
        handleSelect={(title, description, file) => {
          handleCreate(title, description, file);
          popup.close();
        }}
      />,
    )
    .withTitle('Create Artwork');
};

export { openCreateArtworkModal };

export default function CreateArtworkModal({
  handleSelect,
}: {
  handleSelect: (
    title: string,
    description: string | null,
    file: FileInfo,
  ) => void;
}) {
  const [title, setTitle] = useState('');
  const [file, setFile] = useState<FileInfo | null>(null);

  const isDisabled = !title || !file;
  return (
    <div className="flex flex-col gap-10 w-[25vw] max-w-200">
      <Input
        placeholder="Title"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <div className="flex flex-col gap-5">
        <div className="relative flex w-full aspect-square">
          {file && file.url ? (
            <img src={file.url} alt={file.name} />
          ) : (
            <div className="flex items-center justify-center w-full h-full border border-dashed rounded">
              <span className="text-sm text-text-primary">
                No Image Uploaded
              </span>
            </div>
          )}
        </div>
        <div className="self-end">
          <FileUploaderMetadata
            isImage
            isMedia={false}
            onUploadSuccess={(file) => {
              setFile(file);
            }}
          >
            <Button variant="default" size="sm">
              <UploadFile className="w-5 h-5 stroke-neutral-500" />
              Upload Artwork
            </Button>
          </FileUploaderMetadata>
        </div>
      </div>
      <Button
        variant="default"
        size="lg"
        className="bg-primary"
        disabled={isDisabled}
        onClick={() => {
          if (!isDisabled && file) {
            handleSelect(title, null, file);
          }
        }}
      >
        Create Artwork
      </Button>
    </div>
  );
}
