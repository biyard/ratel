'use client';
import { Space, SpaceType } from '@/lib/api/models/spaces';

import { Discuss, Palace, Vote } from '@/components/icons';
import { useState } from 'react';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { route } from '@/route';
import { useRouter } from 'next/navigation';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import RadioButton from '@/components/radio-button';

interface SpaceFormProps {
  type: SpaceType;
  Icon: React.JSX.Element;
  label: string;
  description: string;
  disabled?: boolean;
  experiment?: boolean;
}

const SpaceForms: SpaceFormProps[] = [
  // {
  //   type: SpaceType.Legislation,
  //   Icon: <Palace />,
  //   label: 'Legislation',
  //   description: 'Propose and decide on new rules or policies.',
  //   disabled: true,
  // },
  {
    type: SpaceType.Poll,
    Icon: <Vote />,
    label: 'Poll',
    description: 'Collect quick opinions or preferences.',
    experiment: true,
  },
  {
    type: SpaceType.Deliberation,
    Icon: <Discuss />,
    label: 'Deliberation',
    description: 'Share perspectives and engage in in-depth discussion.',
  },
  {
    type: SpaceType.SprintLeague,
    Icon: <Palace />,
    label: 'Sprint League',
    description:
      'Mini social game where three runners compete in a race, and their speed is determined by community voting',
    experiment: true,
  },

  // {
  //   type: SpaceType.Nft,
  //   Icon: <Cube />,
  //   label: 'NFT',
  //   description: 'Submit information to issue an NFT.',
  //   disabled: true,
  // },
];
export default function SelectSpaceForm({ feed_id }: { feed_id: number }) {
  const [isLoading, setLoading] = useState(false);
  const router = useRouter();
  const popup = usePopup();
  const handleSend = async () => {
    setLoading(true);
    try {
      const res = await apiFetch<Space>(
        `${config.api_url}${ratelApi.spaces.createSpace()}`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            create_space: {
              space_type: selectedType,
              feed_id,
              user_ids: [],
              num_of_redeem_codes: 0,
            },
          }),
        },
      );
      if (res.data) {
        logger.debug('Space created successfully:', res.data.id);
        if (res.data.space_type === SpaceType.Deliberation) {
          router.push(route.deliberationSpaceById(res.data.id));
        }
      }
    } catch (error) {
      logger.error('Error creating space:', error);
    } finally {
      popup.close();
    }
  };
  const [selectedType, setSelectedType] = useState<SpaceType | null>(null);
  return (
    <div className="flex flex-col gap-2.5 p-1.5">
      {SpaceForms.map((form) => (
        <SpaceForm
          key={form.type}
          form={form}
          selected={selectedType === form.type}
          onClick={() => setSelectedType(form.type)}
        />
      ))}
      <LoadablePrimaryButton
        className="w-full mt-4"
        disabled={!selectedType}
        onClick={handleSend}
        isLoading={isLoading}
      >
        Send
      </LoadablePrimaryButton>
    </div>
  );
}

function SpaceForm({
  form,
  selected,
  onClick,
}: {
  form: SpaceFormProps;
  selected: boolean;
  onClick: () => void;
}) {
  const disabled = form.disabled || (form.experiment && !config.experiment);
  return (
    <div
      className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] ${selected ? 'border-primary' : 'border-neutral-800'} ${disabled ? 'opacity-50 cursor-not-allowed' : ''}} `}
      onClick={() => {
        if (!disabled) {
          onClick();
        }
      }}
    >
      <div className="size-8 [&>svg]:size-8">{form.Icon}</div>
      <div className="flex flex-col flex-1 gap-1">
        <span className="font-bold text-[15px]/[20px] text-white">
          {form.label}
        </span>
        <span className="font-normal text-[15px]/[24px] text-neutral-300">
          {form.description}
        </span>
      </div>
      <RadioButton selected={selected} onClick={onClick} />
    </div>
  );
}
