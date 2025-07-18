'use client';
import { useRepost } from './repost-feeds';
import Image from 'next/image';
import { X, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';

export function RepostModal({ onRepostClick }: { onRepostClick?: () => void }) {
  const {
    showRepostModal,
    originalPost,
    cancelRepost,
    submitRepost,
    isSubmitting,
  } = useRepost();

  const handleSubmit = async () => {
    await submitRepost();
    onRepostClick?.(); // To parent update only after success
  };

  if (!showRepostModal || !originalPost) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-neutral-900 border border-neutral-700 rounded-lg w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-neutral-700">
          <h2 className="text-lg font-semibold text-white">Repost</h2>
          <button
            onClick={cancelRepost}
            className="text-neutral-400 hover:text-white transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-4">
          {/* Original post preview */}
          <div className="border-l-4 border-neutral-600 pl-4 bg-neutral-800 rounded-r-lg p-4">
            <div className="flex items-center gap-2 mb-2">
              <Image
                src={originalPost.author_profile_url}
                alt={originalPost.author_name}
                width={24}
                height={24}
                className="rounded-full"
              />
              <span className="text-sm text-neutral-400">
                {originalPost.author_name}
              </span>
            </div>

            <h3 className="font-semibold text-white mb-2">
              {originalPost.title}
            </h3>

            <div
              className="text-neutral-300 text-sm line-clamp-3"
              dangerouslySetInnerHTML={{ __html: originalPost.contents }}
            />

            {originalPost.url && (
              <div className="mt-2">
                <Image
                  src={originalPost.url}
                  alt="Original post image"
                  width={300}
                  height={200}
                  className="rounded-lg object-cover"
                />
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-end gap-3 p-4 border-t border-neutral-700">
          <button
            onClick={cancelRepost}
            className="px-4 py-2 text-neutral-400 hover:text-white transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSubmit}
            disabled={isSubmitting}
            className={cn(
              'px-6 py-2 rounded-lg font-medium transition-all',
              isSubmitting
                ? 'bg-neutral-700 text-neutral-500 cursor-not-allowed'
                : 'bg-primary text-black hover:bg-primary/90',
            )}
          >
            {isSubmitting ? (
              <>
                <Loader2 className="animate-spin inline mr-2" size={16} />
                Reposting...
              </>
            ) : (
              'Repost'
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
