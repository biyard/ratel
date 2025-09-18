import { User } from '../api/models/user';
import { GroupPermission } from './group-permission';

type PermissionValue = number | string | { hi: number; lo: number };

type GroupLike = {
  creator_id: number;
  permissions: PermissionValue;
};

const POW32 = 0x100000000;

function normalizeToHiLo(v: PermissionValue): { hi: number; lo: number } {
  // 이미 {hi, lo}
  if (v && typeof v === 'object' && 'hi' in v && 'lo' in v) {
    const hi = v.hi >>> 0;
    const lo = v.lo >>> 0;
    return { hi, lo };
  }

  if (typeof v === 'number' && Number.isFinite(v) && v >= 0) {
    const hi = Math.floor(v / POW32) >>> 0;
    const lo = v >>> 0;
    return { hi, lo };
  }

  if (typeof v === 'string') {
    const s = v.trim();

    if (/^0x[0-9a-f]+$/i.test(s) || /^[0-9a-f]+$/i.test(s)) {
      const hex = s.startsWith('0x') || s.startsWith('0X') ? s.slice(2) : s;
      const padded = hex.padStart(16, '0');
      const hiHex = padded.slice(0, -8);
      const loHex = padded.slice(-8);
      const hi = parseInt(hiHex, 16) >>> 0;
      const lo = parseInt(loHex, 16) >>> 0;
      return { hi, lo };
    }

    if (/^[0-9]+$/.test(s)) {
      let hi = 0;
      let lo = 0;
      for (let i = 0; i < s.length; i++) {
        const digit = s.charCodeAt(i) - 48;
        const lo10 = lo * 10;
        const carry1 = Math.floor(lo10 / POW32);
        lo = (lo10 >>> 0) + digit;
        let carry2 = 0;
        if (lo >= POW32) {
          lo -= POW32;
          carry2 = 1;
        }
        hi = (hi * 10 + carry1 + carry2) >>> 0;
      }
      return { hi, lo };
    }
  }

  return { hi: 0, lo: 0 };
}

function testBit64(hi: number, lo: number, bitIndex: number): boolean {
  if (bitIndex < 32) {
    return ((lo >>> bitIndex) & 1) === 1;
  }
  const b = bitIndex - 32;
  return ((hi >>> b) & 1) === 1;
}

export default function checkGroupPermission(
  user: User,
  teamId: number,
  permission: GroupPermission,
  teamParentId?: number | null,
): boolean {
  if (!user) return false;

  if (
    user.id === teamId ||
    (teamParentId != null && user.id === teamParentId)
  ) {
    return true;
  }

  const groups = user.groups as GroupLike[] | undefined;
  if (!Array.isArray(groups) || groups.length === 0) return false;

  for (const g of groups) {
    if (g && g.creator_id === teamId) {
      const { hi, lo } = normalizeToHiLo(g.permissions);
      if (testBit64(hi, lo, permission as number)) {
        return true;
      }
    }
  }
  return false;
}
