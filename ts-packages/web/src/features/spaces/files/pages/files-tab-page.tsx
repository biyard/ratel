import React, { useState } from 'react';
import useFilesByLocation from '@/features/spaces/files/hooks/use-files-by-location';
import { FileLocation } from '@/features/spaces/files/types/file';
import { FileListByLocation } from '@/features/spaces/files/components/file-list-by-location';
import { FileUploadWithLocation } from '@/features/spaces/files/components/file-location-selector';
import { useTranslation } from 'react-i18next';
import { useUpdateFileMutation } from '@/features/spaces/files/hooks/use-update-file-mutation';
import FileModel from '@/features/spaces/files/types/file';

interface FilesPageProps {
  spacePk: string;
  isAdmin: boolean;
}

/**
 * Files tab page with location-based filtering
 * Shows all files and allows filtering by location (Overview, Board, Files)
 */
export function FilesPage({ spacePk, isAdmin }: FilesPageProps) {
  const { t } = useTranslation('Space');
  const [selectedLocation, setSelectedLocation] = useState<
    FileLocation | undefined
  >(undefined);
  const { data: filesResponse } = useFilesByLocation(spacePk, selectedLocation);
  const updateFilesMutation = useUpdateFileMutation();

  const handleFileAdded = async (newFile: FileModel) => {
    // Add the new file to the existing files array
    const updatedFiles = [...filesResponse.files, newFile];

    try {
      await updateFilesMutation.mutateAsync({
        spacePk,
        files: updatedFiles,
      });
    } catch (error) {
      console.error('Failed to add file:', error);
    }
  };

  const locationFilters = [
    { value: undefined, label: t('all_files') || 'All Files' },
    { value: FileLocation.Overview, label: t('location_overview') },
    { value: FileLocation.Board, label: t('location_board') },
    { value: FileLocation.Files, label: t('location_files') },
  ];

  return (
    <div className="max-w-4xl mx-auto p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{t('menu_files')}</h1>
      </div>

      {/* Location Filter Tabs */}
      <div className="flex gap-2 border-b">
        {locationFilters.map((filter) => (
          <button
            key={filter.value || 'all'}
            onClick={() => setSelectedLocation(filter.value)}
            className={`px-4 py-2 font-medium border-b-2 transition-colors ${
              selectedLocation === filter.value
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-600 hover:text-gray-900'
            }`}
          >
            {filter.label}
          </button>
        ))}
      </div>

      {/* Files List */}
      <div className="min-h-[200px]">
        <FileListByLocation
          spacePk={spacePk}
          location={selectedLocation}
          onFileClick={(url) => window.open(url, '_blank')}
        />
      </div>

      {/* Upload Section (Admin Only) */}
      {isAdmin && (
        <div className="mt-8 p-6 border-2 border-dashed rounded-lg bg-gray-50">
          <h3 className="text-lg font-semibold mb-4">{t('upload_file')}</h3>
          <FileUploadWithLocation
            onFileAdded={handleFileAdded}
            defaultLocations={[FileLocation.Files]}
          />
        </div>
      )}
    </div>
  );
}
