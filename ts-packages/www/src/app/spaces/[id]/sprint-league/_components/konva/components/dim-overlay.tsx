import { Layer, Rect } from 'react-konva';
import { VIEWPORT_HEIGHT, VIEWPORT_WIDTH } from '../';

export default function DimOverlay({
  visible,
  opacity = 0.5,
}: {
  visible: boolean;
  opacity?: number;
}) {
  if (!visible) return null;
  return (
    <Layer listening={false} name="dim">
      <Rect
        x={0}
        y={0}
        width={VIEWPORT_WIDTH}
        height={VIEWPORT_HEIGHT}
        fill="black"
        opacity={opacity}
      />
    </Layer>
  );
}
