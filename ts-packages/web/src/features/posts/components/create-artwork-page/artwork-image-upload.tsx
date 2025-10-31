import { PhotosUploadPhoto } from '@/components/icons';
import { Col } from '@/components/ui/col';
import { useCreateArtworkPageI18n } from './i18n';

interface ArtworkImageUploadProps {
  imageUrl: string | null;
  onImageUpload: (imageUrl: string) => void;
  t: ReturnType<typeof useCreateArtworkPageI18n>;
}

export function ArtworkImageUpload({
  imageUrl,
  onImageUpload,
  t,
}: ArtworkImageUploadProps) {
  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      // Check file size (10MB limit)
      if (file.size > 10 * 1024 * 1024) {
        alert('File size must be under 10MB');
        return;
      }

      // Check file type
      if (!file.type.startsWith('image/')) {
        alert('Please upload an image file (JPG, PNG)');
        return;
      }

      const reader = new FileReader();
      reader.onloadend = () => {
        onImageUpload(reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();

    const file = e.dataTransfer.files?.[0];
    if (file) {
      // Check file size (10MB limit)
      if (file.size > 10 * 1024 * 1024) {
        alert('File size must be under 10MB');
        return;
      }

      // Check file type
      if (!file.type.startsWith('image/')) {
        alert('Please upload an image file (JPG, PNG)');
        return;
      }

      const reader = new FileReader();
      reader.onloadend = () => {
        onImageUpload(reader.result as string);
      };
      reader.readAsDataURL(file);
    }
  };

  const triggerFileInput = () => {
    const input = document.getElementById(
      'artwork-image-input',
    ) as HTMLInputElement;
    input?.click();
  };

  return (
    <Col className="gap-2.5">
      {/* Upload Interface */}
      <div
        className="aspect-square w-full flex flex-col items-center justify-center gap-3 p-6 rounded-[10px] border border-primary cursor-pointer relative overflow-hidden"
        onClick={triggerFileInput}
        onDragOver={handleDragOver}
        onDrop={handleDrop}
      >
        <input
          id="artwork-image-input"
          type="file"
          accept="image/jpeg,image/png,image/jpg"
          onChange={handleFileChange}
          className="hidden"
        />

        {/* Background Image Layer */}
        {imageUrl ? (
          <>
            <img
              src={imageUrl}
              alt={t.image_label}
              className="w-full h-full object-contain"
            />
          </>
        ) : (
          <div className="flex flex-col items-center gap-3">
            <PhotosUploadPhoto className="size-10 [&>path]:stroke-primary" />
            <Col className="items-center gap-2">
              <div className="text-center text-white text-sm">
                <p>{t.image_label}</p>
                <p>{t.image_formats}</p>
              </div>
            </Col>
          </div>
        )}

        {/* Content on top */}
      </div>

      {/* Info Text */}
      <p className="text-xs ">
        {t.max_resolution}
        <br />
        {t.max_file_size}
      </p>
    </Col>
  );
}
