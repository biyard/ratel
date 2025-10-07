import type React from 'react';
export interface Member {
  id: string;
  name: string;
  avatar: string;
  status?: 'online' | 'offline' | 'away';
}

export interface NavItemType {
  id: string;
  label: string;
  icon: React.ReactNode;
  active?: boolean;
}

export interface SectionType {
  id: string;
  title: string;
  items: string[];
  isOpen: boolean;
}

export interface AttachedFile {
  id: string;
  name: string;
  type: 'pdf' | 'doc' | 'img';
  size: string;
}

export interface Comment {
  id: string;
  author: {
    name: string;
    avatar: string;
  };
  content: string;
  timestamp: string;
  likes: number;
  percentage?: number;
  replies?: Comment[];
}
