import Card from '@/components/card';
import { Edit1, Save } from '@/components/icons';

import { useState } from 'react';
import { PostEditor } from '@/features/posts/components/post-editor';
import { getFileType, toContentType } from '@/lib/file-utils';
import {
  completeMultipartUpload,
  getPutMultiObjectUrl,
  getPutObjectUrl,
} from '@/lib/api/ratel/assets.v3';
import FileModel from '../../files/types/file';
// import { executeOnKeyStroke } from '@/utils/key-event-handle';

async function uploadVideo(file: File) {
  const partSize = 5 * 1024 * 1024;
  const totalParts = Math.ceil(file.size / partSize);
  const fileTypeKey = getFileType(file);

  if (totalParts === 1) {
    const res = await getPutObjectUrl(totalParts, fileTypeKey);
    const presignedUrl = res.presigned_uris[0];
    const publicUrl = res.uris[0];
    const r = await fetch(presignedUrl, {
      method: 'PUT',
      headers: { 'Content-Type': toContentType(fileTypeKey) },
      body: file,
    });
    if (!r.ok) throw new Error('upload failed');
    return { url: publicUrl };
  }

  const res = await getPutMultiObjectUrl(totalParts, fileTypeKey);
  const { presigned_uris, uris, upload_id, key } = res;
  const etags: { etag: string; part_number: number }[] = [];
  for (let i = 0; i < totalParts; i++) {
    const start = i * partSize;
    const end = Math.min(start + partSize, file.size);
    const chunk = file.slice(start, end);
    const rr = await fetch(presigned_uris[i], {
      method: 'PUT',
      body: chunk,
      credentials: 'omit',
    });
    if (!rr.ok) throw new Error(`part ${i + 1} failed`);
    const etag = (rr.headers.get('etag') || '').replaceAll('"', '');
    if (!etag) throw new Error(`part ${i + 1} missing etag`);
    etags.push({ etag, part_number: i + 1 });
  }
  await completeMultipartUpload({ upload_id, key, parts: etags });
  return { url: uris[0] };
}

export default function SpaceHTMLContentEditor({
  files,
  htmlContent,
  url,
  canEdit,
  disabledFileUpload = true,
  // enableEnterToSave = false,
  onContentChange,
  uploadAsset,
  onImageUpload,
  onRemoveImage,
  onUploadPDF,
  onRemovePdf,
}: {
  files?: FileModel[] | null;
  htmlContent: string;
  url: string | null;
  canEdit: boolean;
  enableEnterToSave?: boolean;
  disabledFileUpload?: boolean;
  uploadAsset?: (file: File) => Promise<{ url: string }>;
  onUploadPDF?: (fileList: FileList | File[]) => void;
  onRemovePdf?: (index: number) => void;
  onContentChange: (newContent: string) => void;
  onImageUpload?: (imageUrl: string) => Promise<void>;
  onRemoveImage?: () => Promise<void>;
}) {
  const [isEditing, setEditing] = useState<boolean>(false);
  const [content, setContent] = useState<string>(htmlContent);
  const Icon = !isEditing ? Edit1 : Save;

  // const onKeyDown = (e: React.KeyboardEvent) => {
  //   if (!isEditing || !enableEnterToSave) return;
  //   executeOnKeyStroke(
  //     e,
  //     () => {
  //       setEditing(false);
  //       onContentChange(content);
  //     },
  //     () => setEditing(false),
  //   );
  // };

  return (
    <>
      <Card className="relative pb-20">
        {canEdit && (
          <Icon
            role="button"
            className="absolute right-5 bottom-6 w-5 h-5 [&>path]:stroke-1  text-gray-400 cursor-pointer hover:text-gray-600"
            onClick={() => {
              if (isEditing) {
                onContentChange(content);
                setEditing(false);
              } else if (canEdit) {
                setEditing(true);
              }
            }}
          />
        )}

        <PostEditor
          files={files}
          onUploadPDF={onUploadPDF}
          onRemovePdf={onRemovePdf}
          disabledFileUpload={disabledFileUpload}
          content={content}
          onUpdate={(nextContent) => {
            setContent(nextContent);
          }}
          uploadAsset={uploadAsset}
          uploadVideo={uploadVideo}
          editable={isEditing}
          showToolbar={isEditing}
          onImageUpload={onImageUpload}
          onRemoveImage={onRemoveImage}
          url={url}
          data-pw="space-recommendation-editor"
        />
      </Card>
    </>
  );
}
