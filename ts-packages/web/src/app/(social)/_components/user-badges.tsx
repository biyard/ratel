import { Badge } from '@/lib/api/models/user';
import React from 'react';
import Image from 'next/image';

export interface UserBadgesProps {
  badges: Badge[];
}

export default function UserBadges({ badges }: { badges: Badge[] }) {
  return (
    <div className="grid grid-cols-5 gap-2.5 items-center justify-start">
      {badges.map((badge) => (
        <div className="relative aspect-square" key={`user-badge-${badge.id}`}>
          <Image
            fill
            objectFit="cover"
            alt={`Badge ${badge.name}`}
            src={badge.image_url}
          />
        </div>
      ))}
    </div>
  );
}
