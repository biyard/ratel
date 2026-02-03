import React, { useEffect, useMemo, useRef, useState } from 'react';
import { TFunction } from 'i18next';
import ForceGraph2D, { ForceGraphMethods } from 'react-force-graph-2d';
import { forceCollide, forceCenter } from 'd3-force';
import { NetworkGraph } from '../../polls/types/network-graph';

export type NetworkProps = {
  t: TFunction<'SpacePollAnalyze', undefined>;
  isHtml?: boolean;
  network?: NetworkGraph;
};

type FGNode = {
  id: string;
  degree?: number;
  betweenness?: number;
  rank?: number;
  x?: number;
  y?: number;
};

type FGLink = {
  source: string;
  target: string;
  weight?: number;
};

export function NetworkChart({ t, isHtml, network }: NetworkProps) {
  const fgRef = useRef<ForceGraphMethods<FGNode, FGLink> | null>(null);
  const wrapRef = useRef<HTMLDivElement | null>(null);
  const didInitialFitRef = useRef(false);
  const didSetupRef = useRef(false);
  const [size, setSize] = useState({ w: 0, h: 0 });

  const graph = useMemo(() => {
    const nodesRaw = Array.isArray(network?.nodes) ? network!.nodes : [];
    const edgesRaw = Array.isArray(network?.edges) ? network!.edges : [];

    const nodesBase: FGNode[] = nodesRaw
      .map((n) => ({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        id: String((n as any)?.node ?? '').trim(),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        degree: Number((n as any)?.degree_centrality ?? 0),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        betweenness: Number((n as any)?.betweenness_centrality ?? 0),
      }))
      .filter((n) => n.id.length > 0);

    const scoreOf = (n: FGNode) => Math.max(n.degree ?? 0, n.betweenness ?? 0);
    const rankMap = new Map<string, number>(
      [...nodesBase]
        .sort((a, b) => scoreOf(b) - scoreOf(a))
        .map((n, i) => [n.id, i]),
    );

    const nodes: FGNode[] = nodesBase.map((n) => ({
      ...n,
      rank: rankMap.get(n.id) ?? 9999,
    }));

    const nodeSet = new Set(nodes.map((n) => n.id));

    const links: FGLink[] = edgesRaw
      .map((e) => ({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        source: String((e as any)?.source ?? '').trim(),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        target: String((e as any)?.target ?? '').trim(),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        weight: Number((e as any)?.weight ?? 0),
      }))
      .filter(
        (e) =>
          e.source.length > 0 &&
          e.target.length > 0 &&
          e.source !== e.target &&
          nodeSet.has(e.source) &&
          nodeSet.has(e.target),
      );

    const maxDegree = nodes.reduce((m, n) => Math.max(m, n.degree ?? 0), 0);
    const maxWeight = links.reduce((m, l) => Math.max(m, l.weight ?? 0), 0);

    return { nodes, links, maxDegree, maxWeight };
  }, [network]);

  const getNodeRadius = (n: FGNode) => {
    const r = n.rank ?? 9999;
    if (r === 0) return 52;
    if (r <= 2) return 44;
    if (r <= 5) return 36;
    if (r <= 10) return 28;
    if (r <= 20) return 22;
    return 16;
  };

  const getLinkAlpha = (w?: number) => {
    const ww = w ?? 0;
    if (graph.maxWeight <= 0) return 0.12;
    const r = ww / graph.maxWeight;
    return 0.06 + r * 0.22;
  };

  const graphData = useMemo(
    () => ({ nodes: graph.nodes, links: graph.links }),
    [graph.nodes, graph.links],
  );

  const fitToCenter = (force = false) => {
    const fg = fgRef.current;
    if (!fg || graph.nodes.length === 0) return;
    if (!force && didInitialFitRef.current) return;

    let minX = Infinity,
      maxX = -Infinity,
      minY = Infinity,
      maxY = -Infinity;

    for (const n of graph.nodes) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const x = Number((n as any).x);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const y = Number((n as any).y);
      if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
      if (x < minX) minX = x;
      if (x > maxX) maxX = x;
      if (y < minY) minY = y;
      if (y > maxY) maxY = y;
    }

    fg.zoom(1, 0);
    fg.centerAt(0, 0, 0);

    if (!Number.isFinite(minX)) {
      fg.zoomToFit(300, 80);
      didInitialFitRef.current = true;
      return;
    }

    const cx = (minX + maxX) / 2;
    const cy = (minY + maxY) / 2;

    fg.centerAt(cx, cy, 0);
    fg.zoomToFit(300, 80);

    const z = fg.zoom();
    if (Number.isFinite(z) && z > 1.9) fg.zoom(1.9, 0);

    didInitialFitRef.current = true;
  };

  useEffect(() => {
    const el = wrapRef.current;
    if (!el) return;

    const ro = new ResizeObserver(() => {
      const rect = el.getBoundingClientRect();
      const w = Math.max(0, Math.floor(rect.width));
      const h = Math.max(0, Math.floor(rect.height));
      setSize((prev) => (prev.w === w && prev.h === h ? prev : { w, h }));
    });

    ro.observe(el);
    return () => ro.disconnect();
  }, []);

  useEffect(() => {
    if (didSetupRef.current) return;
    const fg = fgRef.current;
    if (!fg || graph.nodes.length === 0 || size.w === 0 || size.h === 0) return;

    didInitialFitRef.current = false;

    for (const n of graph.nodes) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (n as any).x = undefined;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (n as any).y = undefined;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (n as any).vx = undefined;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (n as any).vy = undefined;
    }

    const chargeStrength = Math.max(
      -700,
      Math.min(-220, -300 - graph.nodes.length * 7),
    );

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const linkForce: any = fg.d3Force('link');
    if (linkForce) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      linkForce.distance((l: any) => {
        const w = Number(l?.weight ?? 0);
        const r = graph.maxWeight > 0 ? w / graph.maxWeight : 0;
        return 170 - r * 70;
      });
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      linkForce.strength((l: any) => {
        const w = Number(l?.weight ?? 0);
        const r = graph.maxWeight > 0 ? w / graph.maxWeight : 0;
        return 0.06 + r * 0.22;
      });
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const chargeForce: any = fg.d3Force('charge');
    if (chargeForce) chargeForce.strength(chargeStrength);

    fg.d3Force('center', forceCenter(0, 0));

    fg.d3Force(
      'collide',
      forceCollide<FGNode>()
        .radius((n) => getNodeRadius(n) + 14)
        .iterations(2),
    );

    fg.d3ReheatSimulation();
    didSetupRef.current = true;

    const raf1 = requestAnimationFrame(() => {
      const raf2 = requestAnimationFrame(() => fitToCenter(true));
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (fitToCenter as any)._raf2 = raf2;
    });
    const to1 = window.setTimeout(() => fitToCenter(true), 120);

    return () => {
      cancelAnimationFrame(raf1);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const raf2 = (fitToCenter as any)._raf2;
      if (raf2) cancelAnimationFrame(raf2);
      window.clearTimeout(to1);
    };
  }, [graph.nodes.length, graph.maxWeight, graph.maxDegree, size.w, size.h]);

  if (!graph.nodes.length) {
    return (
      <div className="w-full rounded-xl border border-input-box-border p-6 text-center text-sm text-text-secondary">
        {t('no_network')}
      </div>
    );
  }

  return (
    <div className="w-full">
      {!isHtml && (
        <div className="mb-3 flex items-center justify-center">
          <div className="text-center text-base font-semibold text-text-primary">
            Text Network
          </div>
        </div>
      )}

      <div
        ref={wrapRef}
        className="relative h-[520px] w-full overflow-hidden rounded-xl"
      >
        {size.w > 0 && size.h > 0 && (
          <ForceGraph2D
            ref={fgRef}
            width={size.w}
            height={size.h}
            graphData={graphData}
            backgroundColor="transparent"
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            linkColor={(l: any) =>
              `rgba(120,120,120,${getLinkAlpha(l?.weight)})`
            }
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            linkWidth={(l: any) => {
              const w = Number(l?.weight ?? 0);
              if (graph.maxWeight <= 0) return 0.8;
              return 0.6 + (w / graph.maxWeight) * 1.2;
            }}
            enableNodeDrag={false}
            enableZoomInteraction={false}
            enablePanInteraction={false}
            warmupTicks={240}
            cooldownTicks={1}
            cooldownTime={0}
            d3VelocityDecay={0.35}
            onEngineStop={() => fitToCenter(true)}
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            nodeCanvasObject={(node: any, ctx, globalScale) => {
              const id = String(node?.id ?? '');
              const r = getNodeRadius(node);
              const x = Number(node?.x ?? 0);
              const y = Number(node?.y ?? 0);

              ctx.beginPath();
              ctx.arc(x, y, r, 0, Math.PI * 2);
              ctx.fillStyle = 'rgba(171, 215, 231, 0.85)';
              ctx.fill();

              ctx.lineWidth = 2 / globalScale;
              ctx.strokeStyle = 'rgba(55, 65, 81, 0.85)';
              ctx.stroke();

              const fontSize =
                Math.max(9, Math.min(14, r * 0.75)) / globalScale;
              ctx.font = `600 ${fontSize}px sans-serif`;
              ctx.fillStyle = 'rgba(17, 24, 39, 0.9)';
              ctx.textAlign = 'center';
              ctx.textBaseline = 'middle';
              ctx.fillText(id, x, y);
            }}
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            nodePointerAreaPaint={(node: any, color, ctx) => {
              const r = getNodeRadius(node);
              ctx.fillStyle = color;
              ctx.beginPath();
              ctx.arc(node.x, node.y, r + 8, 0, Math.PI * 2);
              ctx.fill();
            }}
          />
        )}
      </div>
    </div>
  );
}
