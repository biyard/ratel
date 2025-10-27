// import { useState } from 'react';

// type QrData = {
//   imageUrl?: string;
//   rawUrl?: string;
//   helpText?: string;
// };

// export function usePaymentQrModal() {
//   const [open, setOpen] = useState(false);
//   const [data, setData] = useState<QrData>({});

//   function openPaymentQrModal(d: QrData) {
//     setData(d);
//     setOpen(true);
//   }

//   function Modal() {
//     if (!open) return null;
//     return (
//       <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/60">
//         <div className="w-full max-w-md rounded-2xl bg-white p-6 shadow-xl">
//           <h2 className="mb-3 text-xl font-semibold">결제 QR</h2>
//           {data.helpText && (
//             <p className="mb-4 text-sm text-neutral-600">{data.helpText}</p>
//           )}
//           {data.imageUrl ? (
//             <img
//               src={data.imageUrl}
//               alt="Binance Pay QR"
//               className="mx-auto mb-4 w-64 h-64 object-contain"
//             />
//           ) : (
//             <p className="mb-4 text-sm text-red-600">QR 이미지가 없습니다.</p>
//           )}
//           <div className="flex items-center gap-2">
//             {data.rawUrl && (
//               <a
//                 href={data.rawUrl}
//                 target="_blank"
//                 rel="noreferrer"
//                 className="inline-flex items-center rounded-xl border px-3 py-2 text-sm hover:bg-neutral-50"
//               >
//                 새 탭에서 열기
//               </a>
//             )}
//             <button
//               onClick={() => setOpen(false)}
//               className="ml-auto inline-flex items-center rounded-xl bg-black px-3 py-2 text-sm text-white"
//             >
//               닫기
//             </button>
//           </div>
//         </div>
//       </div>
//     );
//   }

//   return { Modal, openPaymentQrModal };
// }
