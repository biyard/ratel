import React from 'react';
import { FileListByLocation } from '@/features/spaces/files/components/file-list-by-location';
import { FileUploadWithLocation } from '@/features/spaces/files/components/file-location-selector';
import { FileLocation } from '@/features/spaces/files/types/file';
import FileModel from '@/features/spaces/files/types/file';

interface BoardFilesProps {
  spacePk: string;
  isAdmin: boolean;
  onFileAdded?: (file: FileModel) => void;
}

/**
 * Component for displaying and managing files in the Board tab
 * Files with FileLocation.Board will be shown here
 */
export function BoardFiles({
  spacePk,
  isAdmin,
  onFileAdded,
}: BoardFilesProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">Board Files</h3>

      {/* Display files filtered by Board location */}
      <FileListByLocation
        spacePk={spacePk}
        location={FileLocation.Board}
        onFileClick={(url) => window.open(url, '_blank')}
      />

      {/* Allow admins to upload files with location selection */}
      {isAdmin && onFileAdded && (
        <div className="mt-6 p-4 border rounded-lg bg-gray-50">
          <FileUploadWithLocation
            onFileAdded={onFileAdded}
            defaultLocations={[FileLocation.Board, FileLocation.Files]}
          />
        </div>
      )}
    </div>
  );
}
