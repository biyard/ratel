import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import React from 'react';

export default function UnFollowPopup({
  username,
  email,

  oncancel,
  unfollow,
}: {
  username: string;
  email: string;

  oncancel: () => void;
  unfollow: () => void;
}) {
  return (
    <div className="w-100 max-tablet:w-full flex flex-col gap-10 items-center">
      <Col className="w-full gap-2.5">
        <p className="text-center break-words leading-relaxed">
          Unfollow the{' '}
          <span className="font-bold break-all">
            {username ? username : email}
          </span>{' '}
          account.
        </p>
      </Col>

      <Row className="w-full grid grid-cols-2">
        <Button
          variant="rounded_secondary"
          className="w-full"
          onClick={() => {
            oncancel();
          }}
        >
          Cancel
        </Button>
        <Button
          className={'cursor-pointer bg-primary'}
          variant={'rounded_primary'}
          onClick={() => {
            unfollow();
          }}
        >
          UnFollowed
        </Button>
      </Row>
    </div>
  );
}
