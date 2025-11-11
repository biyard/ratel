// import { spaceKeys } from '@/constants';
// import { useMutation, useQueryClient } from '@tanstack/react-query';
// import { Attribute } from '../types/answer-type';
// import { createSpacePanel } from '@/lib/api/ratel/panel.spaces.v3';
// import { ListPanelResponse } from '../types/list-panel-response';
// import { SpacePanelResponse } from '../types/space-panel-response';

// type Vars = {
//   spacePk: string;
//   name: string;
//   quotas: number;
//   attributes: Attribute[];
// };

// export function useCreatePanelMutation() {
//   const qc = useQueryClient();

//   return useMutation({
//     mutationKey: ['create-panel'],
//     mutationFn: async (v: Vars) => {
//       const { spacePk, name, quotas, attributes } = v;
//       await createSpacePanel(spacePk, name, quotas, attributes);
//       return v;
//     },

//     onMutate: async (vars) => {
//       const { spacePk, name, quotas, attributes } = vars;
//       const qk = spaceKeys.panels(spacePk);

//       await qc.cancelQueries({ queryKey: qk });

//       const prev = qc.getQueryData<ListPanelResponse>(qk);

//       const optimisticItem: SpacePanelResponse = {
//         pk: crypto.randomUUID(),
//         name,
//         quotas,
//         attributes,
//       } as SpacePanelResponse;

//       qc.setQueryData<ListPanelResponse>(qk, (old) => {
//         if (!old) {
//           return { panels: [optimisticItem], bookmark: null };
//         }

//         return {
//           ...old,
//           panels: [optimisticItem, ...old.panels],
//         };
//       });

//       return { qk, prev };
//     },

//     onError: (_err, _vars, ctx) => {
//       if (ctx?.qk) qc.setQueryData(ctx.qk, ctx.prev);
//     },

//     onSettled: async (_data, _error, { spacePk }) => {
//       const qk = spaceKeys.panels(spacePk);
//       await qc.invalidateQueries({ queryKey: qk });
//     },
//   });
// }
