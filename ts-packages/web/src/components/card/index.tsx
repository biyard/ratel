import { cn } from '@/lib/utils';
import { cva, type VariantProps } from 'class-variance-authority'; // cva 사용 (버튼처럼)

const cardVariants = cva(
  'flex flex-col w-full justify-start items-start rounded-[10px] px-4 py-5',
  {
    variants: {
      variant: {
        default: 'bg-card-bg-secondary border border-card-border',
        secondary: 'bg-card-bg border border-card-border',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  },
);

export interface CardProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof cardVariants> {
  children: React.ReactNode;
}

const Card = ({ variant, className, children, ...props }: CardProps) => {
  return (
    <div className={cn(cardVariants({ variant }), className)} {...props}>
      {children}
    </div>
  );
};

export default Card;
