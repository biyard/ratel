import useImage from 'use-image';
import { Image as KonvaImage } from 'react-konva';

export default function StoppedImage({
  imageUrl,
  x,
  y,
  scale = 1,
}: {
  imageUrl: string;
  x: number;
  y: number;
  scale?: number;
}) {
  const [image] = useImage(imageUrl);
  if (!image) return null;

  return (
    <KonvaImage
      image={image}
      x={x}
      y={y}
      scale={{ x: scale, y: scale }}
      offset={{ x: image.width / 2, y: image.height / 2 }}
    />
  );
}
