'use client';

import { useState } from 'react';
import { Col } from '@/components/ui/col';
import { useWallet } from '@/lib/api/hooks/wallet';
import { ClipboardCopy } from 'lucide-react';
import Image from 'next/image';
import { Button } from '@/components/ui/button';

export type WalletSummaryProps = {
  onUpdate?: (address: string) => void;
};

export default function WalletSummary({ onUpdate }: WalletSummaryProps) {
  const { address, connectWallet } = useWallet();
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    if (address) {
      navigator.clipboard.writeText(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    }
  };

  return (
    <div className="w-full">
      <Col>
        <div
          className="flex items-center justify-between p-4 bg-component-bg rounded-lg hover:bg-component-bg/70 transition-colors cursor-pointer"
          onClick={connectWallet}
        >
          <div className="flex items-center space-x-3">
            <Image
              src="/logos/meta-mask-icon.png"
              width={40}
              height={40}
              className="w-10 h-10 bg-[#D9D9D9] rounded-full p-2"
              alt="MetaMask"
            />
            <div className="font-semibold">
              <div>MetaMask</div>
              <div className="mt-1 flex items-center space-x-2 text-sm text-gray-400">
                {address ? (
                  <>
                    <span>
                      {address.slice(0, 6)}...{address.slice(-4)}
                    </span>
                    <ClipboardCopy
                      className="w-4 h-4 cursor-pointer hover:text-gray-300"
                      onClick={handleCopy}
                    />
                    {copied && (
                      <span className="text-green-400 text-xs">Copied!</span>
                    )}
                  </>
                ) : (
                  <span>Connect Wallet to Get Started</span>
                )}
              </div>
            </div>
          </div>

          {address && (
            <Button variant={'default'} onClick={() => onUpdate?.(address)}>
              Save
            </Button>
          )}
        </div>

        {/* <Row className="justify-end mt-4 hidden">
        </Row> */}
      </Col>
    </div>
  );
}
