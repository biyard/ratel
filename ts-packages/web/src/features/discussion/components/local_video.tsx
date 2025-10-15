import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { DefaultMeetingSession } from 'amazon-chime-sdk-js';

type FitMode = 'contain' | 'cover';

export default function LocalVideo({
  meetingSession,
  isVideoOn,
}: {
  meetingSession: DefaultMeetingSession;
  isVideoOn: boolean;
}) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const wrapRef = useRef<HTMLDivElement>(null);
  const [started, setStarted] = useState(false);

  const [fit, setFit] = useState<FitMode>('contain');

  const [, setVw] = useState(0);
  const [, setVh] = useState(0);
  const [, setCw] = useState(0);
  const [, setCh] = useState(0);

  const [overflow, setOverflow] = useState({ x: 0, y: 0 });
  const [offset, setOffset] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const av = meetingSession.audioVideo;
    let mounted = true;

    const ensureDevice = async () => {
      let cams = await av.listVideoInputDevices();
      if (!cams.length) {
        try {
          const s = await navigator.mediaDevices.getUserMedia({ video: true });
          s.getTracks().forEach((t) => t.stop());
          cams = await av.listVideoInputDevices();
        } catch {
          return null;
        }
      }
      await av.startVideoInput(cams[0].deviceId);
      return true;
    };

    const start = async () => {
      const ok = await ensureDevice();
      if (!ok || !mounted) return;

      const observer = {
        videoTileDidUpdate: (ts: any) => {
          if (!mounted) return;
          if (ts.localTile && ts.tileId != null && videoRef.current) {
            av.bindVideoElement(ts.tileId, videoRef.current);
          }
        },
      };
      av.addObserver(observer);

      if (isVideoOn) {
        av.startLocalVideoTile();
        const local = av.getLocalVideoTile();
        const id = local?.state().tileId ?? null;
        if (id != null && videoRef.current) {
          av.bindVideoElement(id, videoRef.current);
        }
      }

      setStarted(true);
      return () => av.removeObserver(observer);
    };

    const cleanupObs = start();

    return () => {
      mounted = false;
      av.stopLocalVideoTile();
      Promise.resolve(cleanupObs).then(
        (fn) => typeof fn === 'function' && fn(),
      );
    };
  }, [meetingSession]);

  useEffect(() => {
    const av = meetingSession.audioVideo;
    if (!started) return;
    if (isVideoOn) {
      av.startLocalVideoTile();
      const id = av.getLocalVideoTile()?.state().tileId ?? null;
      if (id != null && videoRef.current) {
        av.bindVideoElement(id, videoRef.current);
      }
    } else {
      av.stopLocalVideoTile();
    }
  }, [isVideoOn, started, meetingSession]);

  // useEffect(() => {
  //   const av = meetingSession.audioVideo;
  //   if (!started) return;
  //   if (isVideoOn) av.startLocalVideoTile();
  //   else av.stopLocalVideoTile();
  // }, [isVideoOn, started, meetingSession.audioVideo]);

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
    vEl.addEventListener('loadedmetadata', update);
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
    const toPct = (val: number, total: number) => {
      if (total <= 0) return 50;
      return (val / total + 0.5) * 100;
    };
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
      className={`absolute inset-0 overflow-hidden bg-black touch-none select-none z-50 ${fit === 'cover' ? 'cursor-grab' : ''}`}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onDoubleClick={onDoubleClick}
      onTouchEnd={onTouchEnd}
    >
      <video
        ref={videoRef}
        id="local-video-element"
        className={`absolute inset-0 w-full h-full ${fit === 'cover' ? 'object-cover' : 'object-contain'}`}
        style={{ objectPosition: objectPos }}
        autoPlay
        muted
        playsInline
        onLoadedMetadata={() => console.log('Video metadata loaded')}
        onCanPlay={() => console.log('Video can play')}
        onPlay={() => console.log('Video playing')}
        onError={(e) => console.error('Video error:', e)}
      />
    </div>
  );
}
