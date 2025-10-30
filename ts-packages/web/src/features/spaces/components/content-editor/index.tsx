import Card from '@/components/card';
import { Edit1, Save } from '@/components/icons';

import { useState } from 'react';
import { TiptapEditor } from '@/components/text-editor';
import { getFileType, toContentType } from '@/lib/file-utils';
import {
  completeMultipartUpload,
  getPutMultiObjectUrl,
  getPutObjectUrl,
} from '@/lib/api/ratel/assets.v3';
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
  htmlContent,
  canEdit,
  // enableEnterToSave = false,
  onContentChange,
}: {
  htmlContent: string;
  canEdit: boolean;
  enableEnterToSave?: boolean;
  onContentChange: (newContent: string) => void;
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
      <Card className="relative">
        {canEdit && (
          <Icon
            role="button"
            className="absolute right-5 bottom-6 w-5 h-5 [&>path]:stroke-1  text-gray-400 cursor-pointer hover:text-gray-600"
            onClick={() => {
              if (isEditing) {
                console.log('save');
                onContentChange(content);
                setEditing(false);
              } else if (canEdit) {
                console.log('Start editing');
                setEditing(true);
              }
            }}
          />
        )}
        <TiptapEditor
          content={content}
          onUpdate={(nextContent) => {
            setContent(nextContent);
          }}
          uploadVideo={uploadVideo}
          editable={isEditing}
          showToolbar={isEditing}
          data-pw="space-recommendation-editor"
        />
      </Card>
    </>
  );
}
