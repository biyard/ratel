import { PlayerImage } from '../types/sprint-league-player';

/**
 * CharacterPreview component displays a static preview of a character
 * Used in the player selection modal to show available characters
 */
export default function CharacterPreview({ images }: { images: PlayerImage }) {
  // Use the win image for preview as it's a static image that shows the character clearly
  return (
    <div className="w-full h-full flex items-center justify-center bg-neutral-900 light:bg-neutral-100">
      {images.win ? (
        <img
          src={images.win}
          alt="Character preview"
          className="w-full h-full object-contain"
        />
      ) : (
        <div className="text-neutral-500 text-sm">No preview available</div>
      )}
    </div>
  );
}
