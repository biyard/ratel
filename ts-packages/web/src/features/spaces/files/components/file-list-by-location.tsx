import React, { Suspense } from 'react';
import { FileLocation } from '@/features/spaces/files/types/file';
import useFilesByLocation from '@/features/spaces/files/hooks/use-files-by-location';
import { useTranslation } from 'react-i18next';

interface FileListByLocationProps {
  spacePk: string;
  location: FileLocation;
  onFileClick?: (fileUrl: string) => void;
}

function FileListContent({
  spacePk,
  location,
  onFileClick,
}: FileListByLocationProps) {
  const { t } = useTranslation('Space');
  const { data: fileResponse } = useFilesByLocation(spacePk, location);

  if (!fileResponse.files || fileResponse.files.length === 0) {
    return (
      <div className="text-gray-500 text-sm text-center py-8">
        {t('no_files_in_location')}
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {fileResponse.files.map((file, index) => (
        <div
          key={file.id || index}
          className="flex items-center justify-between p-3 border rounded hover:bg-gray-50 cursor-pointer"
          onClick={() => file.url && onFileClick?.(file.url)}
        >
          <div className="flex items-center gap-3">
            <div className="text-2xl">
              {file.ext === 'pdf' && '📄'}
              {['jpg', 'png', 'gif'].includes(file.ext || '') && '🖼️'}
              {['doc', 'docx', 'word'].includes(file.ext || '') && '📝'}
              {['xls', 'xlsx', 'excel'].includes(file.ext || '') && '📊'}
              {['zip'].includes(file.ext || '') && '🗜️'}
            </div>
            <div>
              <div className="font-medium text-sm">{file.name}</div>
              {file.description && (
                <div className="text-xs text-gray-500">{file.description}</div>
              )}
              <div className="text-xs text-gray-400">
                {file.size}
                {file.uploaded_at &&
                  ` • ${new Date(file.uploaded_at).toLocaleDateString()}`}
              </div>
            </div>
          </div>
          <div className="text-xs text-gray-400 uppercase">{file.ext}</div>
        </div>
      ))}
    </div>
  );
}

export function FileListByLocation(props: FileListByLocationProps) {
  return (
    <Suspense fallback={<div className="animate-pulse">Loading files...</div>}>
      <FileListContent {...props} />
    </Suspense>
  );
}
