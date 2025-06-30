import { config } from '@/config';
import { Star } from 'lucide-react';
import React from 'react';

export default function UserTier() {
  return (
    <div className="mt-4 aria-hidden:hidden" aria-hidden={!config.experiment}>
      <div className="flex justify-between items-center">
        <span className="text-sm">Tier</span>
        <div className="flex items-center gap-1">
          <span className="text-sm">Diamond</span>
          <div className="w-4 h-4 rounded-full bg-[#fcb300] flex items-center justify-center">
            <Star size={10} className="text-black" />
          </div>
        </div>
      </div>
      <div className="mt-1 h-1 w-full bg-gray-700 rounded-full">
        <div className="h-full w-full bg-[#fcb300] rounded-full"></div>
      </div>
    </div>
  );
}
