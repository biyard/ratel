import { Button } from '@/components/ui/button';
import { Col } from '@/components/ui/col';
import { Input } from '@/components/ui/input';
import { Row } from '@/components/ui/row';
import GoogleIcon from '@/assets/icons/google.svg?react';
// import { TelegramIcon } from '@/components/icons';
import { route } from '@/route';
import Link from 'next/link';
import { useState } from 'react';
import Footer from './footer';

//FIXME: add telegram login

const LoginBox = ({
  icon,
  label,
  onClick,
}: {
  icon: React.ReactNode;
  label: string;
  onClick: () => Promise<void>;
}) => {
  return (
    <button
      className="flex flex-row w-full rounded-[10px] bg-[#000203] px-5 py-5.5 gap-5 cursor-pointer items-center"
      onClick={onClick}
    >
      {icon}
      <div className="font-semibold text-white text-base">{label}</div>
    </button>
  );
};

interface LoginFormProps {
  onGoogleLogin: () => Promise<void>;
  onTelegramLogin: () => Promise<void>;
  onLogin: (username: string, password: string) => Promise<void>;
}
export default function LoginForm({
  onGoogleLogin,
  onTelegramLogin: _onTelegramLogin,
  onLogin,
}: LoginFormProps) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  return (
    <div className="flex flex-col w-full gap-5">
      <Row className="justify-start items-center text-sm gap-1">
        <label className="text-white font-medium">New to Ratel?</label>
        <Link href={route.signup()}>
          <button className="text-primary/70 hover:text-primary">
            Create an account
          </button>
        </Link>
      </Row>

      <Col className="gap-4">
        <Col>
          <label className="text-sm text-white">Email address</label>
          <Input
            type="email"
            name="username"
            autoComplete="email"
            placeholder="Enter your email address"
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </Col>

        <Col>
          <label className="text-sm text-white">Password</label>
          <Input
            type="password"
            placeholder="Enter your password"
            className="w-full bg-[#000203] rounded-[10px] px-5 py-5.5 text-white font-light"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </Col>

        <Row className="justify-end items-center text-sm">
          <Button
            variant="rounded_secondary"
            size="sm"
            onClick={async () => {
              await onLogin(email, password);
            }}
          >
            Sign In
          </Button>
        </Row>
      </Col>

      <div className="rule-with-text align-center text-center font-light text-white">
        Or
      </div>

      <div className="flex flex-col gap-2.5">
        <LoginBox
          icon={<GoogleIcon />}
          label="Continue With Google"
          onClick={async () => {
            await onGoogleLogin();
          }}
        />
        {/* <LoginBox
          icon={<TelegramIcon width={24} height={24} />}
          label="Continue With Telegram"
          onClick={async () => {
            await onTelegramLogin();
          }}
        /> */}
      </div>
      <Footer />
    </div>
  );
}
