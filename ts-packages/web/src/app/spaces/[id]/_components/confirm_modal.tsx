

export default function PublishModal({ onClose }: { onClose: () => void }) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60">
      <div className="relative w-[360px] bg-[#1E1E1E] text-white rounded-lg shadow-lg p-6">
        {/* Close Button */}
        <button
          className="absolute top-3 right-4 text-gray-400 hover:text-gray-200 text-xl"
          onClick={onClose}
        >
          &times;
        </button>

        {/* Header */}
        <h2 className="text-lg font-semibold mb-2">Save first, make public?</h2>

        {/* Description */}
        <p className="text-sm text-gray-400 mb-4 leading-relaxed">
          Looks like you haven’t saved yet. <br />
          Want to save your changes before going public, <br />
          or skip it and publish anyway?
        </p>

        {/* Warning */}
        <p className="text-xs text-gray-500 mb-6">
          Once made public, this Space will be visible to everyone <br />
          and <span className="font-semibold">cannot be made private again.</span>
        </p>

        {/* Buttons */}
        <div className="flex justify-between items-center">
          <button
            className="text-sm text-gray-200 hover:underline"
            onClick={onClose}
          >
            Just Publish
          </button>
          <button className="bg-[#FFD02F] text-black text-sm font-medium px-4 py-2 rounded-md hover:bg-yellow-400">
            Save & Publish
          </button>
        </div>
      </div>
    </div>
  );
}
