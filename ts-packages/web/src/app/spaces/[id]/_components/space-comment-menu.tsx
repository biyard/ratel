'use client';
import { useState } from 'react';
import CommentIcon from '@/assets/icons/comment.svg';
import SearchIcon from '@/assets/icons/search.svg';
import HamburgerIcon from '@/assets/icons/hamburger2.svg';
import Check from '@/assets/icons/check-dynamic.svg';
import CheckCircle from '@/assets/icons/check-circle.svg';

interface CommentProps {
  author: string;
  mention: string;
  text: string;
  time: string;
  replies: number;
  avatarGroup: string[];
  status: 'done' | 'pending';
  highlighted: boolean;
  id: number;
}

export default function SideCommentMenu() {
  const [comments] = useState<CommentProps[]>([
    {
      id: 1,
      author: 'Hackartist',
      mention: '@Username',
      text: 'Could you check this up real quick.',
      time: '1w ago',
      replies: 3,
      avatarGroup: [
        'https://i.pravatar.cc/40?img=1',
        'https://i.pravatar.cc/40?img=2',
        'https://i.pravatar.cc/40?img=3',
      ],
      status: 'done',
      highlighted: false,
    },
    {
      id: 2,
      author: 'Hackartist',
      mention: '@Username',
      text: 'Could you check this up real quick.',
      time: '1w ago',
      replies: 3,
      avatarGroup: [
        'https://i.pravatar.cc/40?img=1',
        'https://i.pravatar.cc/40?img=2',
        'https://i.pravatar.cc/40?img=3',
      ],
      status: 'pending',
      highlighted: true,
    },
    {
      id: 3,
      author: 'Hackartist',
      mention: '@Username',
      text: 'Could you check this up real quick.',
      time: '1w ago',
      replies: 3,
      avatarGroup: [
        'https://i.pravatar.cc/40?img=1',
        'https://i.pravatar.cc/40?img=2',
        'https://i.pravatar.cc/40?img=3',
      ],
      status: 'done',
      highlighted: false,
    },
  ]);

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-2.5">
      <div className="border border-card-border rounded-[10px] bg-card-bg-secondary border">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="font-semibold flex items-center gap-2 text-text-primary">
            <CommentIcon /> Comments
          </h2>
          <HamburgerIcon />
        </div>

        {/* Search */}
        <div className="px-4 py-2">
          <div className="flex items-center bg-write-comment-box-bg rounded-lg px-3 py-2">
            <SearchIcon className=" mr-2" />
            <input
              type="text"
              placeholder="Search"
              className="bg-write-comment-box-bg outline-none text-sm w-full placeholder:text-modal-label-text text-modal-label-text"
            />
          </div>
        </div>

        {/* Comments */}
        <div className="flex-1 overflow-y-auto space-y-2 p-4">
          {comments.map((comment, idx) => (
            <CommentBox
              key={idx}
              id={comment.id}
              author={comment.author}
              mention={comment.mention}
              text={comment.text}
              time={comment.time}
              replies={comment.replies}
              avatarGroup={comment.avatarGroup}
              status={comment.status}
              highlighted={comment.highlighted}
            />
          ))}
        </div>
      </div>
    </div>
  );
}

function CommentBox({
  author,
  mention,
  text,
  time,
  replies,
  avatarGroup,
  status,
}: CommentProps) {
  return (
    <div
      className={`p-3 rounded-xl  flex flex-col gap-1 relative hover:bg-button-bg `}
    >
      {/* Avatars + status */}
      <div className="flex items-center justify-between">
        <div className="flex -space-x-2">
          {avatarGroup.map((src, idx) => (
            <img
              key={idx}
              src={src}
              alt="avatar"
              className="w-6 h-6 rounded-full "
            />
          ))}
        </div>
        {status === 'done' ? (
          <CheckCircle />
        ) : (
          <Check className="bg-follow-button-bg-secondary rounded-full" />
        )}
      </div>

      {/* Text */}
      <div className="text-sm cursor-pointer">
        <div className=" text-text-primary flex items-center justify-between">
          {author}{' '}
          <span className="text-[12px] text-title-text font-normal">
            {time}
          </span>
        </div>

        <p className="text-text-primary">
          <span className="text-home-side font-medium">{mention}</span> {text}
        </p>
      </div>

      {/* Footer */}
      <div className="flex justify-between text-xs text-gray-500 mt-1">
        <span className="text-home-side cursor-pointer">{replies} replies</span>
      </div>
    </div>
  );
}
