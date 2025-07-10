
'use client';
import { useState, useRef, useEffect } from 'react';
import {
  Edit1,
  ThumbUp,
  Shares,
  Extra,
  UnlockIcon,
  CommentIcon,
  LockIcon,
  SaveIcon,
} from '@/components/icons';
import { SpaceConfirmModal } from '@/components/popup/space-confirm-popup';
import { Eye } from 'lucide-react';
import { useRouter } from 'next/navigation';
import { showSuccessToast } from '@/lib/toast';

import { useParams } from 'next/navigation';
import { DeleteConfirmModal } from '@/components/popup/delete-confirm-popup';


export default function StatsBar({
  handleEdit,
  handleSave,
  isEdit,
  isSave,
}: {
  isEdit: boolean;
  isSave: boolean;
  handleEdit: () => void;
  handleSave: () => void;
}) {
  const [showMenu, setShowMenu] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);

  const {id} = useParams()

  const menuRef = useRef<HTMLDivElement>(null);
  const [publish, setPublish] = useState(false);
  const router = useRouter();

  const handlepublic = () => {
    setPublish(true);
  };

  // Close menu on outside click
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setShowMenu(false);
      }
    }

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  const menuItems = [
    {
      name: 'Manage committee',
      action: () => {
        router.push(`/spaces/${id}/committe`);
      },
    },
    {
      name: 'Change form',
      action: () => {
        router.push(`/spaces/${id}/change-form`);
      },
    },
    {
      name: 'Change Category',
      action: () => {
        router.push(`/spaces/${id}/change-category`);
      },
    },
    {
      name: 'Delete',
      action: () =>  setShowDeleteModal(true),
    },
  ];

  return (
    <div className="bg-background text-white px-0 md:px-4 py-2 space-y-4 flex flex-col items-center justify-between relative">
      {/* Actions - buttons */}
      <div className="flex flex-col md:flex-row md:justify-end ml-auto gap-2 relative">
        {isEdit ? (
          <button
            onClick={handleSave}
            className="flex bg-white text-[#18181B] text-[16px] px-3 py-1.5 rounded-md hover:bg-gray-200 font-medium items-center"
          >
            <SaveIcon />
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
          onClick={handlepublic}
          className="bg-white text-[#18181B] text-[16px] px-3 py-1.5 rounded-md border hover:bg-gray-200 border-gray-600 flex items-center gap-1"
        >
          <UnlockIcon />
          Make Public
        </button>

        <SpaceConfirmModal
          show={publish}
          onClose={() => setPublish(false)}
          onConfirm={() => {
            alert('Public!');
            setPublish(false);
          }}
          title={
            isEdit && !isSave
              ? 'Save first, go public?'
              : 'You’re About to Go Public'
          }
          description={
            isEdit && !isSave
              ? 'Looks like you haven’t saved yet. Want to save your changes before going public, or skip it and publish anyway?'
              : 'Once made public, this Sprint will be visible to everyone and'
          }
          emphasisText="cannot be made private again."
          confirmText={isEdit && !isSave ? 'Save & Publish' : 'Make Public'}
          cancelText="Cancel"
        />

        {/* Extra Button */}
        <div className="relative" ref={menuRef}>
          <button
            onClick={() => setShowMenu((prev) => !prev)}
            className="p-2 bg-neutral-800 rounded-md"
          >
            <Extra />
          </button>

          {/* Popover Menu */}
          {showMenu && (
            <div className="absolute right-0 mt-2 w-48 bg-[#2B2B2B] text-white rounded-md shadow-lg py-2 z-50 space-y-1 text-sm">
              {menuItems.map((item, index) => (
                <button
                  key={index}
                  onClick={() => {
                    item.action();
                    setShowMenu(false);
                  }}
                  className="w-full text-left px-4 py-2 hover:bg-[#3a3a3a]"
                >
                  {item.name}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
       
      {/* Delete confirm modal */}
      <DeleteConfirmModal
        show={showDeleteModal}
        onClose={() => setShowDeleteModal(false)}
        onConfirm={() => {
          //  delete logic here
          console.log('Item deleted');
          setShowDeleteModal(false);
          showSuccessToast("Space deleted successfully!")
        }}
        title="Confirm Deletion"
        description="Are you absolutely sure you want to delete this item?"
        emphasisText=" This action cannot be undone."
        confirmText="Yes, Delete"
      />

      {/* Stats Section */}
      <div className="flex items-center gap-6 text-sm text-gray-300">
        <div className="flex items-center gap-1">
          <ThumbUp />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <CommentIcon />
          <span>201</span>
        </div>
        <div className="flex items-center gap-1">
          <Eye size={16} className="text-gray-400" />
          <span>221K</span>
        </div>
        <div className="flex items-center gap-1">
          <Shares />
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
