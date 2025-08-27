import { X } from 'lucide-react';
interface UploadedImage {
  id: string;
  src: string;
  name: string;
}

export const ImagePreviewGallery = ({
  uploadedImages,
  insertImageFromPreview,
  removeImage,
}: {
  uploadedImages: UploadedImage[];
  insertImageFromPreview: (src: string) => void;
  removeImage: (id: string) => void;
}) => {
  if (uploadedImages.length === 0) return null;

  return (
    <div className="bg-gray-800 border border-gray-600 rounded-lg p-4">
      <h3 className="text-sm font-medium text-gray-300 mb-3">
        Uploaded Images
      </h3>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
        {uploadedImages.map((image) => (
          <div key={image.id} className="relative group">
            <div className="relative overflow-hidden rounded-lg bg-gray-700">
              <img
                src={image.src}
                alt={image.name}
                className="w-full h-20 object-cover cursor-pointer hover:opacity-80"
                onClick={() => insertImageFromPreview(image.src)}
              />
              <button
                onClick={() => removeImage(image.id)}
                className="absolute -top-1 -right-1 w-5 h-5 bg-red-500 hover:bg-red-600 rounded-full flex items-center justify-center text-white text-xs opacity-0 group-hover:opacity-100"
              >
                <X className="w-3 h-3" />
              </button>
            </div>
            <p className="text-xs text-gray-400 mt-1 truncate">{image.name}</p>
          </div>
        ))}
      </div>
      <p className="text-xs text-gray-500 mt-2">
        Click on an image to insert it into the editor
      </p>
    </div>
  );
};
