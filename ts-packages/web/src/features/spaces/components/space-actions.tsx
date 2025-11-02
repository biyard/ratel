import { BottomTriangle } from '@/components/icons';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Row } from '@/components/ui/row';
import { useState } from 'react';

export type SpaceActionsProps = React.HTMLAttributes<HTMLDivElement> & {
  actions: SpaceActionProp[];
};

export function SpaceActions({ actions }: SpaceActionsProps) {
  const [hold, setHold] = useState(-1);

  return (
    <>
      {hold === -1 ? (
        <SpaceActionDropdown actions={actions} onChange={(n) => setHold(n)} />
      ) : (
        <Button
          aria-role={actions[hold].holdingLabel}
          variant="rounded_primary"
          className="w-full"
          onClick={() => {
            setHold(-1);
            actions[hold].onClickWhileHolding();
          }}
        >
          {actions[hold].holdingLabel}
        </Button>
      )}
    </>
  );
}

export function SpaceActionDropdown({
  actions,
  onChange,
}: SpaceActionsProps & { onChange: (selected: number) => void }) {
  if (actions.length === 0) return null;
  const firstAction = actions[0];

  if (actions.length === 1) {
    return (
      <Button
        aria-role={firstAction.label}
        variant="rounded_secondary"
        className="w-full !important text-[#262626]"
        onPointerDown={(e) => {
          e.stopPropagation();
          e.preventDefault();
        }}
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          if (firstAction.holdingLabel) onChange(0);

          firstAction.onClick();
        }}
      >
        {firstAction.label}
      </Button>
    );
  }

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Row className="gap-0.5">
            <div className="w-full">
              <Button
                aria-role={firstAction.label}
                variant="rounded_secondary"
                className="w-full rounded-r-none !important text-[#262626]"
                onPointerDown={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                }}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  if (firstAction.holdingLabel) onChange(0);

                  firstAction.onClick();
                }}
              >
                {firstAction.label}
              </Button>
            </div>

            <Row className="justify-center items-center px-3 bg-white rounded-full rounded-l-none cursor-pointer w-fit hover:bg-neutral-200">
              <BottomTriangle aria-role="Expand Space Actions" />
            </Row>
          </Row>
        </DropdownMenuTrigger>

        <DropdownMenuContent className="bg-white border-0" align="end">
          {actions.slice(1).map((action, index) => (
            <DropdownMenuItem
              aria-role={action.label}
              key={index}
              onSelect={() => {
                if (action.holdingLabel) onChange(index + 1);
                action.onClick();
              }}
              className="font-bold text-black hover:text-black hover:bg-neutral-200"
            >
              {action.label}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );
}

export type SpaceActionProp = {
  label: string;
  onClick: () => Promise<void>;
  onClickWhileHolding?: () => Promise<void>;
  holdingLabel?: string;
};
