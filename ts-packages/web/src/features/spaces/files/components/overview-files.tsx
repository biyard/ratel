import React from 'react';
import { FileListByLocation } from '@/features/spaces/files/components/file-list-by-location';
import { FileUploadWithLocation } from '@/features/spaces/files/components/file-location-selector';
import { FileLocation } from '@/features/spaces/files/types/file';
import FileModel from '@/features/spaces/files/types/file';

interface OverviewFilesProps {
  spacePk: string;
  isAdmin: boolean;
  onFileAdded?: (file: FileModel) => void;
}

/**
 * Component for displaying and managing files in the Overview tab
 * Files with FileLocation.Overview will be shown here
 */
export function OverviewFiles({
  spacePk,
  isAdmin,
  onFileAdded,
}: OverviewFilesProps) {
  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">Files</h3>

      {/* Display files filtered by Overview location */}
      <FileListByLocation
        spacePk={spacePk}
        location={FileLocation.Overview}
        onFileClick={(url) => window.open(url, '_blank')}
      />

      {/* Allow admins to upload files with location selection */}
      {isAdmin && onFileAdded && (
        <div className="mt-6 p-4 border rounded-lg bg-gray-50">
          <FileUploadWithLocation
            onFileAdded={onFileAdded}
            defaultLocations={[FileLocation.Overview, FileLocation.Files]}
          />
        </div>
      )}
    </div>
  );
}
