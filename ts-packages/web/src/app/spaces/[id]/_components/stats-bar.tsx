// components/StatsBar.tsx
import { ThumbsUp, MessageCircle, Eye, RefreshCcw, Lock, Globe } from 'lucide-react';

export default function StatsBar() {
  return (
    <div className="bg-black text-white px-4 py-2 flex items-center justify-between border-t border-gray-800">
      
      {/* Left side - stats */}
      <div className="flex items-center gap-6 text-sm text-gray-300">
        <div className="flex items-center gap-1">
          <ThumbsUp size={16} className="text-gray-400" />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <MessageCircle size={16} className="text-gray-400" />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <Eye size={16} className="text-gray-400" />
          <span>221K</span>
        </div>
        <div className="flex items-center gap-1">
          <RefreshCcw size={16} className="text-gray-400" />
          <span>403</span>
        </div>
        <div className="flex items-center gap-1">
          <Lock size={16} className="text-gray-400" />
          <span>Private</span>
        </div>
      </div>

      {/* Right side - buttons */}
      <div className="flex items-center gap-2">
        <button className="bg-white text-black text-sm px-3 py-1.5 rounded-md hover:bg-gray-200 font-medium">
          Edit
        </button>
        <button className="bg-gray-800 text-white text-sm px-3 py-1.5 rounded-md hover:bg-gray-700 border border-gray-600 flex items-center gap-1">
          <Globe size={14} />
          Make Public
        </button>
      </div>
    </div>
  );
}
