
export default function DropdownMenu({ onclose, ondelete}: {onclose:() => void; ondelete:() => void;}) {
  return (
    <div className="w-56 bg-neutral-800 text-white rounded-lg shadow-lg py-2 space-y-1">
      <div className="px-4 py-2 hover:bg-neutral-700 cursor-pointer rounded-md text-white text-[14px] font-semibold">
        See committee list
      </div>
      <div className="px-4 py-2 hover:bg-neutral-700 cursor-pointer rounded-md text-white text-[14px] font-semibold">
        Change Category
      </div>
      <div onClick={ondelete} className="px-4 py-2 hover:bg-neutral-700 cursor-pointer rounded-md text-white text-[14px] font-semibold">
        Delete
      </div>
    </div>
  );
}
