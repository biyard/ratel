import { useTranslation } from 'react-i18next';
import { Col } from '@/components/ui/col';
import Card from '@/components/card';
import { SpacePathProps } from '@/features/space-path-props';
import { i18nArtTwinViewer } from './art-twin-viewer-i18n';
import SpaceArtworkTradeTable from '../../../components/space-artwork-trade-table';
import useSuspenseSpaceArtwork from '../../../hooks/use-space-artwork';
import useInfiniteSpaceArtworkTrade from '../../../hooks/use-space-artwork-history';
import { useCallback, useRef } from 'react';
import SpaceArtworkTrade from '../../../types/space-artwork-trade';

export default function SpaceArtNftArtTwinViewerPage({
  spacePk,
}: SpacePathProps) {
  const { data: artwork } = useSuspenseSpaceArtwork(spacePk);
  const {
    data: trades,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteSpaceArtworkTrade(spacePk);
  const { t, i18n } = useTranslation('ArtTwinViewer');
  const observer = useRef<IntersectionObserver | null>(null);

  const lastTradeRef = useCallback(
    (node: HTMLDivElement) => {
      if (isFetchingNextPage) return;
      if (observer.current) observer.current.disconnect();
      observer.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && hasNextPage) {
          fetchNextPage();
        }
      });
      if (node) observer.current.observe(node);
    },
    [isFetchingNextPage, fetchNextPage, hasNextPage],
  );

  // Register i18n
  if (!i18n.hasResourceBundle(i18n.language, 'ArtTwinViewer')) {
    i18n.addResourceBundle(
      'en',
      'ArtTwinViewer',
      i18nArtTwinViewer.en,
      true,
      true,
    );
    i18n.addResourceBundle(
      'ko',
      'ArtTwinViewer',
      i18nArtTwinViewer.ko,
      true,
      true,
    );
  }
  const flattedTrades =
    trades?.pages.flatMap((page) =>
      page.items.map((item) => new SpaceArtworkTrade(item)),
    ) ?? [];
  console.log('flattedTrades', flattedTrades);
  return (
    <Card variant="secondary">
      <Col className="gap-4">
        {!artwork ? (
          <div className="text-center py-8 text-gray-500">
            {t('not_minted_yet')}
          </div>
        ) : (
          <>
            {flattedTrades.length > 0 && (
              <SpaceArtworkTradeTable items={flattedTrades} />
            )}
            <div ref={lastTradeRef} className="h-4" />
            {isFetchingNextPage && (
              <div className="text-center py-4 text-gray-500">
                {t('loading')}
              </div>
            )}
            {!hasNextPage && flattedTrades.length > 0 && (
              <div className="text-center py-4 text-gray-400">
                🎉 {t('end_of_history')}
              </div>
            )}
          </>
        )}
      </Col>
    </Card>
  );
}
