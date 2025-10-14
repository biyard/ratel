'use client';

import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { DefaultMeetingSession, VideoTileState } from 'amazon-chime-sdk-js';

type FitMode = 'contain' | 'cover';
type TileStateAny = VideoTileState | any;

export default function RemoteContentShareVideo({
  meetingSession,
  onRemoteContentTileUpdate,
}: {
  meetingSession: DefaultMeetingSession;
  onRemoteContentTileUpdate: (tileState: TileStateAny | null) => void;
}) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const wrapRef = useRef<HTMLDivElement>(null);
  const boundContentTileIdRef = useRef<number | null>(null);
  const lastReportedTileIdRef = useRef<number | null>(null);
  const cbRef = useRef(onRemoteContentTileUpdate);
  cbRef.current = onRemoteContentTileUpdate;

  const [fit, setFit] = useState<FitMode>('contain');
  const [, setVw] = useState(0);
  const [, setVh] = useState(0);
  const [, setCw] = useState(0);
  const [, setCh] = useState(0);
  const [overflow, setOverflow] = useState({ x: 0, y: 0 });
  const [offset, setOffset] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const av = meetingSession.audioVideo;
    if (!av) return;

    const bindExistingRemoteContent = () => {
      const tiles = av.getAllVideoTiles?.() || [];
      for (const t of tiles) {
        const s = t.state();
        if (
          s.isContent &&
          !s.localTile &&
          s.tileId != null &&
          videoRef.current
        ) {
          av.bindVideoElement(s.tileId, videoRef.current);
          boundContentTileIdRef.current = s.tileId;
          if (lastReportedTileIdRef.current !== s.tileId) {
            lastReportedTileIdRef.current = s.tileId;
            cbRef.current(s);
          }
          break;
        }
      }
    };

    const observer = {
      videoTileDidUpdate: (tileState: VideoTileState) => {
        const isRemoteContent = tileState.isContent && !tileState.localTile;
        const tileId = tileState.tileId ?? null;

        if (!isRemoteContent) return;

        if (
          tileId != null &&
          videoRef.current &&
          boundContentTileIdRef.current !== tileId
        ) {
          av.bindVideoElement(tileId, videoRef.current);
          boundContentTileIdRef.current = tileId;
        }

        if (lastReportedTileIdRef.current !== tileId) {
          lastReportedTileIdRef.current = tileId;
          cbRef.current(tileState);
        }
      },
      videoTileWasRemoved: (tileId: number) => {
        if (boundContentTileIdRef.current === tileId) {
          try {
            av.unbindVideoElement(tileId);
          } catch {}
          if (videoRef.current) videoRef.current.srcObject = null;
          boundContentTileIdRef.current = null;
          if (lastReportedTileIdRef.current !== null) {
            lastReportedTileIdRef.current = null;
            cbRef.current(null);
          }
          setOffset({ x: 0, y: 0 });
          setOverflow({ x: 0, y: 0 });
        }
      },
    };

    bindExistingRemoteContent();
    av.addObserver(observer);
    return () => {
      av.removeObserver(observer);
      const bound = boundContentTileIdRef.current;
      if (bound != null) {
        try {
          av.unbindVideoElement(bound);
        } catch {}
        if (videoRef.current) videoRef.current.srcObject = null;
        boundContentTileIdRef.current = null;
        lastReportedTileIdRef.current = null;
      }
    };
  }, [meetingSession]); // 콜백을 의존성에서 제거

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
      setOffset((prev) => {
        const hx = ox / 2,
          hy = oy / 2;
        return {
          x: Math.max(-hx, Math.min(hx, prev.x)),
          y: Math.max(-hy, Math.min(hy, prev.y)),
        };
      });
    };

    const ro = new ResizeObserver(update);
    ro.observe(wEl);
    vEl.addEventListener('loadedmetadata', update);
    update();

    return () => {
      ro.disconnect();
      vEl.removeEventListener('loadedmetadata', update);
    };
  }, []);

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
    setOffset((prev) => {
      const ox = overflow.x,
        oy = overflow.y;
      const hx = ox / 2,
        hy = oy / 2;
      return {
        x: Math.max(-hx, Math.min(hx, prev.x + dx)),
        y: Math.max(-hy, Math.min(hy, prev.y + dy)),
      };
    });
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
      className={`absolute inset-0 overflow-hidden bg-black touch-none select-none ${fit === 'cover' ? 'cursor-grab' : ''}`}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onDoubleClick={onDoubleClick}
      onTouchEnd={onTouchEnd}
    >
      <video
        id="remote-content-video"
        ref={videoRef}
        className={`absolute inset-0 w-full h-full ${fit === 'cover' ? 'object-cover' : 'object-contain'} z-0`}
        style={{ objectPosition: objectPos }}
        autoPlay
        muted
        playsInline
      />
    </div>
  );
}
