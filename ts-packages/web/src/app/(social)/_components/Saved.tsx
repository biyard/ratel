import { config } from '@/config';
import { ChevronDown, ChevronRight, MessageSquare } from 'lucide-react';
import React from 'react';

export default function Saved() {
  return (
    <div className="mt-6 px-4" hidden={!config.experiment}>
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1">
          <MessageSquare size={14} />
          <h3 className="text-sm font-medium">Saved</h3>
        </div>
        <ChevronDown size={14} />
      </div>
      <div className="mt-2 text-xs text-gray-400">
        <div className="py-1">Crypto/DAO Treasury Transpare...</div>
        <div className="py-1">Crypto/DAO Act Investor</div>
        <div className="py-1">Crypto/DAO Welcome to Protec...</div>
        <div className="mt-1 flex items-center">
          <span>View all</span>
          <ChevronRight size={14} />
        </div>
      </div>
    </div>
  );
}
