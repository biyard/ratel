import React from 'react';
import ReactDOM from 'react-dom';
import RemoveIcon from '@/assets/icons/remove.svg';

interface ModalProps {
  title: string;
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

export const Modal = ({ title, isOpen, onClose, children }: ModalProps) => {
  if (!isOpen) return null;

  return ReactDOM.createPortal(
    <div
      className="fixed top-0 left-0 w-screen h-screen bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]"
      onClick={onClose}
    >
      <div
        className="relative rounded-[12px] p-[25px] min-w-[350px] max-[500px]:w-full max-[500px]:mx-[20px] bg-[#1a1a1a]"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex flex-row w-full justify-between items-center mb-35">
          <div className="font-bold text-white text-[20px]">{title}</div>
          <RemoveIcon
            className="[&>path]:stroke-neutral-80 cursor-pointer"
            width="24"
            height="24"
            onClick={onClose}
          ></RemoveIcon>
        </div>
        {children}
      </div>
    </div>,
    document.getElementById('modal-root') as HTMLElement,
  );
};
