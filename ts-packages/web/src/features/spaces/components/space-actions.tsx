import { Expand } from '@/components/icons';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Row } from '@/components/ui/row';

export type SpaceActionsProps = React.HTMLAttributes<HTMLDivElement> & {
  actions: SpaceActionProp[];
};

export function SpaceActions({ actions }: SpaceActionsProps) {
  const firstAction = actions[0];

  return (
    <>
      <DropdownMenu>
        <Row>
          <div className="w-full">
            <Button
              variant="rounded_secondary"
              className="w-full"
              onClick={firstAction.onClick}
            >
              {firstAction.label}
            </Button>
          </div>

          <DropdownMenuTrigger asChild>
            <div className="w-20">
              <ArrowDown />
            </div>
          </DropdownMenuTrigger>
        </Row>
        <DropdownMenuContent>
          {actions.map((action, index) => (
            <DropdownMenuItem key={index} onSelect={action.onClick}>
              {action.label}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );
}

export function SpaceAction(action: SpaceActionProp) {}

export type SpaceActionProp = {
  label: string;
  onClick: () => Promise<void>;
  onClickWhileHolding: () => Promise<void>;
  holdingLabel?: string;
};
