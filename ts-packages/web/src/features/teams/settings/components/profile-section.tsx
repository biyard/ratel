import FileUploader from '@/features/spaces/files/components/file-uploader';

interface ProfileSectionProps {
  profileUrl: string;
  onProfileUrlChange: (url: string) => void;
  uploadLogoText: string;
}

export function ProfileSection({
  profileUrl,
  onProfileUrlChange,
  uploadLogoText,
}: ProfileSectionProps) {
  return (
    <FileUploader
      onUploadSuccess={onProfileUrlChange}
      data-pw="team-profile-uploader"
    >
      {profileUrl ? (
        <img
          src={profileUrl}
          alt="Team Logo"
          width={80}
          height={80}
          className="w-40 h-40 rounded-full object-cover cursor-pointer"
          data-pw="team-profile-image"
        />
      ) : (
        <button
          className="w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50"
          data-pw="team-profile-upload-button"
        >
          {uploadLogoText}
        </button>
      )}
    </FileUploader>
  );
}
