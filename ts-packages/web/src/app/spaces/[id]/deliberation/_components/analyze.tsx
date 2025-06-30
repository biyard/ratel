'use client';
import { ArrowLeft } from 'lucide-react';
import React from 'react';
import { useDeliberationSpaceContext } from '../provider.client';

export default function AnalyzePage() {
  const { handleGoBack, handleDownloadExcel, answer, survey } =
    useDeliberationSpaceContext();

  return (
    <div className="flex flex-col w-full">
      <div
        className="cursor-pointer w-fit h-fit mb-[20px]"
        onClick={() => {
          handleGoBack();
        }}
      >
        <ArrowLeft width={24} height={24} />
      </div>

      <div className="flex flex-row w-full justify-end">
        <div className="w-fit">
          <button
            className="w-full px-[20px] py-[10px] rounded-[10px] bg-[#fcb300] hover:bg-[#ca8f00] text-black text-bold text-[16px] hover:text-black cursor-pointer"
            disabled={false}
            onClick={() => {
              handleDownloadExcel();
            }}
          >
            {'Download Excel'}
          </button>
        </div>
      </div>
    </div>
  );
}
