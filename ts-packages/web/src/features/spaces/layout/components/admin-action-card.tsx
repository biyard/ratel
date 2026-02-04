import Card from '@/components/card';
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
import { LayoutAction } from '../use-space-layout-controller';

export type AdminCardProps = React.HTMLAttributes<HTMLDivElement> & {
  title: string;
  description: string;
  actions: LayoutAction[];
};
export function CardSkeleton({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: React.ReactNode;
}) {
  return (
    <Card
      variant="default"
      rounded="default"
      className="gap-2.5 bg-divider px-3 py-4 border-[0.5px] border-card-enable-border"
    >
      <div className="flex flex-col gap-1 w-full">
        <h3 className="font-bold text-[15px]/[18px] text-text-primary">
          {title}
        </h3>
        <p className="font-medium text-[13px]/[20px] text-text-secondary">
          {description}
        </p>
      </div>
      {children}
    </Card>
  );
}
export default function AdminActionCard({
  actions,
  title,
  description,
}: AdminCardProps) {
  const [hold, setHold] = useState(-1);

  return (
    <CardSkeleton title={title} description={description}>
      {hold === -1 ? (
        <ActionDropdown
          actions={actions.map((action) => ({
            label: action.label,
            onClick: action.onClick,
          }))}
          onChange={(n) => setHold(n)}
        />
      ) : (
        <></>
      )}
    </CardSkeleton>
  );
}

function ActionDropdown({
  actions,
  onChange,
}: {
  actions: ActionProp[];
  onChange: (selected: number) => void;
}) {
  if (actions.length === 0) return null;
  const firstAction = actions[0];

  if (actions.length === 1) {
    return (
      <Button
        aria-role={firstAction.label}
        variant="rounded_secondary"
        className="w-full"
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
    <div className="w-full">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Row className="gap-0.5 w-full">
            <Button
              aria-role={firstAction.label}
              data-testid="space-action-button"
              variant="rounded_secondary"
              className="rounded-r-none flex-1"
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

            <Row className="justify-center items-center px-3 rounded-full rounded-l-none cursor-pointer bg-btn-secondary-bg w-fit hover:bg-btn-secondary-hover-bg">
              <BottomTriangle
                className="[&>path]:fill-btn-secondary-text [&>path]:stroke-btn-secondary-text"
                aria-role="Expand Space Actions"
              />
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
    </div>
  );
}

export type ActionProp = {
  label: string;
  onClick: () => Promise<void> | void;
  onClickWhileHolding?: () => Promise<void>;
  holdingLabel?: string;
};
