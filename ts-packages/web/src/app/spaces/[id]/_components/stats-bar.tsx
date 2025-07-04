// components/StatsBar.tsx
import {
  ThumbsUp,
  MessageCircle,
  Eye,
  RefreshCcw,
  Lock,
  Globe,
} from 'lucide-react';

import {
  Edit1,
  ThumbUp,
  Rewards,
  Shares,
  Extra,
  UnlockIcon,
  LockIcon,
} from '@/components/icons';
import { useDeliberationSpaceContext } from '../deliberation/provider.client';

interface StatsProps {
  handleEdit: () => void;
  handlePublic: () => void;
  handleMenu: () => void;
  handleSave: () => void;
}

export default function StatsBar({
  handleEdit,
  handlePublic,
  handleMenu,
  handleSave
}: StatsProps) {
  const { isEdit } = useDeliberationSpaceContext();

  return (
    <div className="bg-background text-white px-4 py-2 space-y-4 flex flex-col items-center justify-between">
      {/* Actions - buttons */}
      <div className="flex justify-end ml-auto gap-2">
        {isEdit ? (
          <button onClick={handleSave} className="flex bg-white text-[#18181B] text-[16px] px-3 py-1.5 rounded-md hover:bg-gray-200 font-medium items-center">
            <Edit1 />
            Save
          </button>
        ) : (
          <button
            onClick={handleEdit}
            className="flex bg-white text-[#18181B] text-[16px] px-3 py-1.5 rounded-md hover:bg-gray-200 font-medium items-center"
          >
            <Edit1 />
            Edit
          </button>
        )}

        <button
          onClick={handlePublic}
          className="bg-white text-[#18181B] text-[16px] px-3 py-1.5 rounded-md  border hover:bg-gray-200 border-gray-600 flex items-center gap-1"
        >
          <UnlockIcon />
          Make Public
        </button>

        <button className="p-2 bg-neutral-700 rounded-md">
          <Extra />
        </button>
      </div>

      {/* Left side - stats */}
      <div className="flex items-center gap-6 text-sm text-gray-300">
        <div className="flex items-center gap-1">
          <ThumbUp />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <Shares />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <Eye size={16} className="text-gray-400" />
          <span>221K</span>
        </div>
        <div className="flex items-center gap-1">
          <Rewards />
          <span>403</span>
        </div>
        <div className="flex items-center gap-1">
          <LockIcon />
          <span>Private</span>
        </div>
      </div>
    </div>
  );
}
