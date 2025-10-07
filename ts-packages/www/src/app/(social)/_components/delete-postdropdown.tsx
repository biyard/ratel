'use client';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Trash2 } from 'lucide-react';
import { Extra } from '@/components/icons';
import { useRouter } from 'next/navigation';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export interface DeletePostDropdownProps {
  postId: number;
  userId: number;
  authorId: number;
}

export default function DeletePostDropdown({
  postId,
  userId,
  authorId,
}: DeletePostDropdownProps) {
  const router = useRouter();
  const { post: apiPost } = useApiCall();

  const handleDeletePost = async () => {
    try {
      await apiPost(ratelApi.feeds.removeDraft(postId), { delete: {} });
      router.refresh();
      showSuccessToast('Post deleted successfully');
    } catch (error) {
      console.error('Failed to delete post:', error);
      showErrorToast('Failed to delete post. Please try again.');
    }
  };

  if (userId !== authorId) return null;

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger>
        <button
          onClick={(e) => e.stopPropagation()}
          className="p-1 hover:bg-gray-700 rounded-full focus:outline-none transition-colors"
          aria-haspopup="true"
          aria-label="Post options"
        >
          <Extra className="w-6 h-6 text-gray-400" />
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        className="w-40 bg-[#404040] border-gray-700 transition ease-out duration-100"
      >
        <DropdownMenuItem asChild>
          <button
            onClick={(e) => {
              e.stopPropagation();
              handleDeletePost();
            }}
            className="flex items-center w-full px-4 py-2 text-sm text-red-400 hover:bg-gray-700 cursor-pointer"
          >
            <Trash2 className="w-4 h-4 mr-2" />
            Delete
          </button>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
