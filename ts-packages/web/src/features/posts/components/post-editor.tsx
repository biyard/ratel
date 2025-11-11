import { TiptapEditor } from '@/components/text-editor';
import { forwardRef } from 'react';
import {
  DEFAULT_ENABLED_FEATURES,
  TiptapEditorProps,
} from '@/components/text-editor/types';
import { Editor } from '@tiptap/react';
import FileModel from '@/features/spaces/files/types/file';
import EditableFile from '@/features/spaces/files/components/space-file-editor/editable-file';
import SpaceFile from '@/features/spaces/files/components/space-file-viewer/space-file';
import { downloadPdfFromUrl } from '@/lib/pdf-utils';

export interface PostEditorProps extends TiptapEditorProps {
  url: string | null;
  files?: FileModel[] | null;
  disabledFileUpload?: boolean;
  disabledImageUpload?: boolean;
  onUploadPDF?: (fileList: FileList | File[]) => void;
  onRemovePdf?: (index: number) => void;
  onRemoveImage?: () => void;
}
export const PostEditor = forwardRef<Editor | null, PostEditorProps>(
  (props, ref) => {
    const {
      files,
      url,
      onRemoveImage,
      onRemovePdf,
      editable,
      disabledFileUpload = true,
      disabledImageUpload = false,
      ...editorProps
    } = props;
    let features = DEFAULT_ENABLED_FEATURES;
    if (disabledFileUpload) {
      features = { ...features, pdf: false };
    }

    if (disabledImageUpload) {
      features = { ...features, image: false };
    }

    const handlePdfDownload = async (file: FileModel) => {
      await downloadPdfFromUrl({
        url: file.url ?? '',
        fileName: file.name,
      });
    };

    return (
      <div className="flex flex-col w-full">
        <TiptapEditor
          ref={ref}
          editable={editable}
          enabledFeatures={features}
          {...editorProps}
        />
        {url && (
          <div className="px-2 relative">
            <div className="aspect-video relative">
              <img
                src={url}
                alt="Uploaded image"
                className="object-cover w-full rounded-[8px]"
                sizes="100vw"
              />
              {editable && (
                <button
                  onClick={onRemoveImage}
                  className="absolute top-2 right-2 w-8 h-8 bg-black/60 hover:bg-black/80 hover:scale-110 rounded-full flex items-center justify-center text-white transition-all duration-200"
                  aria-label="Remove image"
                >
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      d="M12 4L4 12M4 4L12 12"
                      stroke="currentColor"
                      strokeWidth="2"
                      strokeLinecap="round"
                    />
                  </svg>
                </button>
              )}
            </div>
          </div>
        )}

        {files && files.length > 0 && (
          <div className="px-2 mt-3 space-y-2">
            {files.map((f, i) => {
              return editable ? (
                <EditableFile
                  key={i}
                  file={f}
                  onclick={() => {
                    onRemovePdf(i);
                  }}
                />
              ) : (
                <SpaceFile
                  key={i}
                  file={f}
                  onclick={async () => {
                    await handlePdfDownload(f);
                  }}
                />
              );
            })}
          </div>
        )}
      </div>
    );
  },
);
