import React from 'react';

import { CheckCircle2 } from '../icons';
type CustomToastProps = {
  message: string;
};

const CustomToast: React.FC<CustomToastProps> = ({ message }) => (
  <div className="px-8 py-6 bg-[#3B82F6E5] rounded-xl ">
    <span className="flex flex-row gap-x-2 items-center  text-white font-bold">
      <CheckCircle2 />
      {message}
    </span>
  </div>
);

export default CustomToast;
