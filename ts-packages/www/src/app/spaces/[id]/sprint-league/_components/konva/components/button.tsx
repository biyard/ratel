import { Group, Image as KonvaImage, Rect } from 'react-konva';
import useImage from 'use-image';

const BackButtonImage = '/images/sprint_league/button_back.png';
const StartButtonImage = '/images/sprint_league/button_start.png';
const VoteButtonImage = '/images/sprint_league/button_vote.png';

type KonvaImageProps = React.ComponentProps<typeof KonvaImage>;

interface ButtonBaseProps extends Omit<KonvaImageProps, 'image'> {
  imageUrl: string;
  disabled?: boolean;
}

function ButtonBase({ imageUrl, disabled, ...props }: ButtonBaseProps) {
  const [image] = useImage(imageUrl);
  if (!image) {
    return null;
  }
  const { x, onClick, ...rest } = props;
  return (
    <Group x={x}>
      <KonvaImage
        image={image}
        {...rest}
        onTap={onClick}
        onClick={disabled ? undefined : onClick}
        onMouseEnter={(e) => {
          const stage = e.target.getStage();
          if (stage && stage.container().style) {
            stage.container().style.cursor = 'pointer';
          }
        }}
        onMouseOut={(e) => {
          const stage = e.target.getStage();
          if (stage && stage.container().style) {
            stage.container().style.cursor = 'default';
          }
        }}
      />
      {disabled && (
        <Rect
          x={5}
          y={5}
          width={image.width - 10}
          height={image.height - 10}
          fill="black"
          opacity={0.25}
          listening={false}
        />
      )}
    </Group>
  );
}

export const StartButton = (
  props: Omit<KonvaImageProps, 'image'> & { disabled?: boolean },
) => <ButtonBase imageUrl={StartButtonImage} {...props} />;
export const BackButton = (
  props: Omit<KonvaImageProps, 'image'> & { disabled?: boolean },
) => <ButtonBase imageUrl={BackButtonImage} {...props} />;
export const VoteButton = (
  props: Omit<KonvaImageProps, 'image'> & { disabled?: boolean },
) => <ButtonBase imageUrl={VoteButtonImage} {...props} />;
