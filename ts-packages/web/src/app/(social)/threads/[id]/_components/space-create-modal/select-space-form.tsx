'use client';
import { Space, SpaceType } from '@/lib/api/models/spaces';

import { Discuss } from '@/components/icons';
import { useState } from 'react';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { route } from '@/route';
import { useRouter } from 'next/navigation';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';

interface SpaceFormProps {
  type: SpaceType;
  Icon: React.JSX.Element;
  label: string;
  description: string;
  disabled?: boolean;
}

const SpaceForms: SpaceFormProps[] = [
  // {
  //   type: SpaceType.Legislation,
  //   Icon: <Palace />,
  //   label: 'Legislation',
  //   description: 'Propose and decide on new rules or policies.',
  //   disabled: true,
  // },
  // {
  //   type: SpaceType.Poll,
  //   Icon: <Vote />,
  //   label: 'Poll',
  //   description: 'Collect quick opinions or preferences.',
  //   disabled: true,
  // },
  {
    type: SpaceType.Deliberation,
    Icon: <Discuss />,
    label: 'Deliberation',
    description: 'Share perspectives and engage in in-depth discussion.',
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
  return (
    <div
      className={`flex flex-row gap-2.5 justify-center items-center w-full p-5 border rounded-[10px] ${selected ? 'border-primary' : 'border-neutral-800'} ${form.disabled ? 'opacity-50 cursor-not-allowed' : ''}} `}
      onClick={() => {
        if (!form.disabled) {
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

function RadioButton({
  onClick,
  selected,
}: {
  onClick: () => void;
  selected: boolean;
}) {
  return (
    <div className="flex items-center">
      <button
        onClick={onClick}
        className={`w-6 h-6 rounded-full flex items-center justify-center transition-colors ${
          selected
            ? 'bg-[#fcb300] hover:bg-[#fcb300]/90'
            : 'border-2 border-[#6b6b6b] hover:border-white'
        }`}
      >
        {selected && (
          <svg
            className="w-3 h-3 text-black"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clipRule="evenodd"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
