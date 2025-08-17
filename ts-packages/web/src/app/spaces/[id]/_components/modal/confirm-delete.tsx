'use client';

import React, { useState } from 'react';

export default function DeleteSpacePopup({
  spaceName,
  onDelete,
  onClose,
}: {
  spaceName?: string;
  onDelete: () => void;
  onClose?: () => void;
}) {
  const [inputValue, setInputValue] = useState('');
  const [isConfirmed, setIsConfirmed] = useState(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setInputValue(value);
    setIsConfirmed(value === spaceName);
  };

  return (
    <div className="w-[500px] flex flex-col mt-6">
      <div className="text-center font-bold text-white text-[24px] mb-6">
        Delete Space '{spaceName}'
      </div>

      <div className="text-center font-medium text-neutral-400 text-base mb-6">
        This action cannot be undone. This will permanently delete the Space and all its contents.
      </div>

      <div className="mb-6">
        <label htmlFor="spaceNameVerification" className="block text-neutral-400 text-sm mb-2">
          To confirm, type the Space name exactly as shown:
        </label>
        <input
          id="spaceNameVerification"
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          className="w-full p-3 bg-neutral-800 border border-neutral-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-primary"
          placeholder={`Type "${spaceName}" to confirm`}
        />
      </div>

      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onClose}
          className="px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400 hover:text-white transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={onDelete}
          disabled={!isConfirmed}
          className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
            isConfirmed
              ? 'bg-red-600 text-white hover:bg-red-700'
              : 'bg-neutral-700 text-neutral-500 cursor-not-allowed'
          } transition-colors`}
        >
          Delete Space
        </button>
      </div>
    </div>
  );
}