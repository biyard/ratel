import React, { useState } from 'react';
import FileModel, {
  FileExtension,
  FileLocation,
} from '@/features/spaces/files/types/file';
import { useTranslation } from 'react-i18next';

interface FileLocationSelectorProps {
  selectedLocations: FileLocation[];
  onChange: (locations: FileLocation[]) => void;
}

export function FileLocationSelector({
  selectedLocations,
  onChange,
}: FileLocationSelectorProps) {
  const { t } = useTranslation('Space');

  const locations = [
    { value: FileLocation.Overview, label: t('location_overview') },
    { value: FileLocation.Board, label: t('location_board') },
    { value: FileLocation.Files, label: t('location_files') },
  ];

  const toggleLocation = (location: FileLocation) => {
    if (selectedLocations.includes(location)) {
      onChange(selectedLocations.filter((l) => l !== location));
    } else {
      onChange([...selectedLocations, location]);
    }
  };

  return (
    <div className="flex gap-2 flex-wrap">
      {locations.map((loc) => (
        <label
          key={loc.value}
          className="flex items-center gap-2 cursor-pointer"
        >
          <input
            type="checkbox"
            checked={selectedLocations.includes(loc.value)}
            onChange={() => toggleLocation(loc.value)}
            className="rounded border-gray-300"
          />
          <span className="text-sm">{loc.label}</span>
        </label>
      ))}
    </div>
  );
}

interface FileUploadWithLocationProps {
  onFileAdded: (file: FileModel) => void;
  defaultLocations?: FileLocation[];
}

export function FileUploadWithLocation({
  onFileAdded,
  defaultLocations = [FileLocation.Files],
}: FileUploadWithLocationProps) {
  const { t } = useTranslation('Space');
  const [selectedLocations, setSelectedLocations] =
    useState<FileLocation[]>(defaultLocations);

  const handleFileUpload = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0];
    if (!file) return;

    // Get file extension
    const fileExt =
      file.name.split('.').pop()?.toLowerCase() || FileExtension.None;

    // Create file model with selected locations
    const fileModel: FileModel = {
      name: file.name,
      size: `${Math.round(file.size / 1024)}KB`,
      ext: (fileExt in FileExtension
        ? (fileExt as FileExtension)
        : FileExtension.None) as FileExtension,
      url: URL.createObjectURL(file),
      locations: selectedLocations,
      uploaded_at: Date.now(),
    };

    onFileAdded(fileModel);
    event.target.value = '';
  };

  return (
    <div className="space-y-4">
      <div>
        <label className="block text-sm font-medium mb-2">
          {t('file_locations')}
        </label>
        <FileLocationSelector
          selectedLocations={selectedLocations}
          onChange={setSelectedLocations}
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          {t('upload_file')}
        </label>
        <input
          type="file"
          onChange={handleFileUpload}
          className="block w-full text-sm text-gray-500
            file:mr-4 file:py-2 file:px-4
            file:rounded-full file:border-0
            file:text-sm file:font-semibold
            file:bg-blue-50 file:text-blue-700
            hover:file:bg-blue-100"
        />
      </div>
    </div>
  );
}
