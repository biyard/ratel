import { CheckCircle2 } from '../icons';

type CustomToastProps = {
  message: string;
};

const CustomToast: React.FC<CustomToastProps> = ({ message }) => (
  <div className="py-6 px-8 rounded-xl bg-[#3B82F6E5]">
    <span className="flex flex-row gap-x-2 items-center font-bold text-white">
      <CheckCircle2 />
      {message}
    </span>
  </div>
);

export default CustomToast;
