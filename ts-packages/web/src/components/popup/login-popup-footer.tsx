'use client';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface LoginPopupFooterProps {}

export const LoginPopupFooter = ({}: LoginPopupFooterProps) => {
  return (
    <div className="flex flex-row w-full justify-center items-center gap-2.5">
      <div className="cursor-pointer text-neutral-400 text-xs/3.5 font-medium">
        Privacy Policy
      </div>
      <div className="cursor-pointer text-neutral-400 text-xs/3.5 font-medium">
        Terms of Service
      </div>
    </div>
  );
};
