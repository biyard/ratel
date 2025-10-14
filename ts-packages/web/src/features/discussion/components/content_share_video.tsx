'use client';

import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { DefaultMeetingSession, VideoTileState } from 'amazon-chime-sdk-js';

type FitMode = 'contain' | 'cover';

export default function ContentShareVideo({
  meetingSession,
  initialFit = 'contain',
}: {
  meetingSession: DefaultMeetingSession;
  initialFit?: FitMode;
}) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const wrapRef = useRef<HTMLDivElement>(null);
  const boundContentTileIdRef = useRef<number | null>(null);

  const [, setVw] = useState(0);
  const [, setVh] = useState(0);
  const [, setCw] = useState(0);
  const [, setCh] = useState(0);

  const [fit, setFit] = useState<FitMode>(initialFit);
  const [overflow, setOverflow] = useState({ x: 0, y: 0 });
  const [offset, setOffset] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const av = meetingSession.audioVideo;
    console.log('[CS] effect mount');

    const bindExistingLocalContent = () => {
      const tiles = av.getAllVideoTiles?.() || [];
      console.log(
        '[CS] existing tiles:',
        tiles.map((t) => t.state()),
      );
      for (const t of tiles) {
        const s = t.state();
        if (
          s.isContent &&
          s.localTile &&
          s.tileId != null &&
          videoRef.current
        ) {
          console.log('[CS] bind existing local content tileId=', s.tileId);
          av.bindVideoElement(s.tileId, videoRef.current);
          boundContentTileIdRef.current = s.tileId;
          setTimeout(() => {
            const v = videoRef.current!;
            console.log(
              '[CS] after bind existing readyState=',
              v.readyState,
              'size=',
              v.videoWidth,
              v.videoHeight,
              'currentSrc=',
              v.currentSrc,
            );
          }, 300);
          break;
        }
      }
    };

    const obs = {
      videoTileDidUpdate: (s: VideoTileState) => {
        console.log('[CS] didUpdate:', s);
        if (!(s.isContent && s.localTile)) return;
        const id = s.tileId ?? null;
        if (
          id != null &&
          videoRef.current &&
          boundContentTileIdRef.current !== id
        ) {
          console.log('[CS] bind on update tileId=', id);
          av.bindVideoElement(id, videoRef.current);
          boundContentTileIdRef.current = id;
          setTimeout(() => videoRef.current?.play().catch(() => {}), 0);
          setTimeout(() => {
            const v = videoRef.current!;
            console.log(
              '[CS] after bind update readyState=',
              v.readyState,
              'size=',
              v.videoWidth,
              v.videoHeight,
              'currentSrc=',
              v.currentSrc,
            );
          }, 300);
        }
      },
      videoTileWasRemoved: (tileId: number) => {
        console.log('[CS] tile removed:', tileId);
        if (boundContentTileIdRef.current === tileId) {
          try {
            av.unbindVideoElement(tileId);
          } catch {}
          if (videoRef.current) videoRef.current.srcObject = null;
          boundContentTileIdRef.current = null;
          setOffset({ x: 0, y: 0 });
          setOverflow({ x: 0, y: 0 });
        }
      },
    };

    bindExistingLocalContent();
    av.addObserver(obs);

    const logTimer = setInterval(() => {
      const tiles = av.getAllVideoTiles?.() || [];
      const content = tiles
        .map((t) => t.state())
        .find((s) => s.isContent && s.localTile);
      console.log(
        '[CS] poll content exist=',
        !!content,
        'bound=',
        boundContentTileIdRef.current,
      );
      const v = videoRef.current;
      if (v)
        console.log(
          '[CS] video readyState=',
          v.readyState,
          'size=',
          v.videoWidth,
          v.videoHeight,
          'currentSrc=',
          v.currentSrc,
        );
    }, 1500);

    return () => {
      console.log('[CS] effect cleanup');
      clearInterval(logTimer);
      av.removeObserver(obs);
      const bound = boundContentTileIdRef.current;
      if (bound != null) {
        try {
          av.unbindVideoElement(bound);
        } catch {}
        if (videoRef.current) videoRef.current.srcObject = null;
        boundContentTileIdRef.current = null;
      }
    };
  }, [meetingSession]);

  useLayoutEffect(() => {
    const vEl = videoRef.current;
    const wEl = wrapRef.current;
    if (!vEl || !wEl) return;

    const update = () => {
      const _vw = vEl.videoWidth || 0;
      const _vh = vEl.videoHeight || 0;
      const _cw = wEl.clientWidth || 1;
      const _ch = wEl.clientHeight || 1;

      setVw(_vw);
      setVh(_vh);
      setCw(_cw);
      setCh(_ch);

      if (!_vw || !_vh) {
        setOverflow({ x: 0, y: 0 });
        setOffset({ x: 0, y: 0 });
        return;
      }

      const scale = Math.max(_cw / _vw, _ch / _vh);
      const scaledW = _vw * scale;
      const scaledH = _vh * scale;
      const ox = Math.max(0, Math.round(scaledW - _cw));
      const oy = Math.max(0, Math.round(scaledH - _ch));
      setOverflow({ x: ox, y: oy });
      setOffset((prev) => clampOffset(prev, ox, oy));
    };

    const ro = new ResizeObserver(update);
    ro.observe(wEl);
    vEl.addEventListener('loadedmetadata', () => {
      console.log(
        '[CS] video loadedmetadata w×h=',
        vEl.videoWidth,
        vEl.videoHeight,
      );
      update();
    });
    update();

    return () => {
      ro.disconnect();
      vEl.removeEventListener('loadedmetadata', update);
    };
  }, []);

  const clampOffset = (o: { x: number; y: number }, ox: number, oy: number) => {
    const hx = ox / 2,
      hy = oy / 2;
    return {
      x: Math.max(-hx, Math.min(hx, o.x)),
      y: Math.max(-hy, Math.min(hy, o.y)),
    };
  };

  const objectPos = (() => {
    if (fit === 'contain') return '50% 50%';
    const toPct = (val: number, total: number) =>
      total <= 0 ? 50 : (val / total + 0.5) * 100;
    return `${toPct(offset.x, overflow.x)}% ${toPct(offset.y, overflow.y)}%`;
  })();

  const dragging = useRef(false);
  const last = useRef({ x: 0, y: 0 });

  const onPointerDown = (e: React.PointerEvent) => {
    if (fit !== 'cover') return;
    dragging.current = true;
    last.current = { x: e.clientX, y: e.clientY };
    (e.currentTarget as Element).setPointerCapture?.(e.pointerId);
  };
  const onPointerMove = (e: React.PointerEvent) => {
    if (fit !== 'cover' || !dragging.current) return;
    const dx = e.clientX - last.current.x;
    const dy = e.clientY - last.current.y;
    last.current = { x: e.clientX, y: e.clientY };
    setOffset((prev) =>
      clampOffset({ x: prev.x + dx, y: prev.y + dy }, overflow.x, overflow.y),
    );
  };
  const onPointerUp = (e: React.PointerEvent) => {
    if (fit !== 'cover') return;
    dragging.current = false;
    (e.currentTarget as Element).releasePointerCapture?.(e.pointerId);
  };

  const lastTap = useRef(0);
  const toggleFit = () => {
    setOffset({ x: 0, y: 0 });
    setFit((f) => (f === 'contain' ? 'cover' : 'contain'));
  };
  const onDoubleClick = toggleFit;
  const onTouchEnd = () => {
    const now = Date.now();
    if (now - lastTap.current < 300) toggleFit();
    lastTap.current = now;
  };

  return (
    <div
      ref={wrapRef}
      className={`absolute inset-0 overflow-hidden bg-black touch-none select-none z-30 ${fit === 'cover' ? 'cursor-grab' : ''}`}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onDoubleClick={onDoubleClick}
      onTouchEnd={onTouchEnd}
    >
      <video
        ref={videoRef}
        className={`absolute inset-0 w-full h-full ${fit === 'cover' ? 'object-cover' : 'object-contain'}`}
        style={{ objectPosition: objectPos }}
        autoPlay
        muted
        playsInline
        onPlay={() => {
          const v = videoRef.current!;
          console.log(
            '[CS] onPlay readyState=',
            v.readyState,
            'size=',
            v.videoWidth,
            v.videoHeight,
          );
        }}
        onError={(e) => {
          const v = videoRef.current!;
          console.log(
            '[CS] video error',
            e,
            'readyState=',
            v?.readyState,
            'currentSrc=',
            v?.currentSrc,
          );
        }}
      />
    </div>
  );
}
