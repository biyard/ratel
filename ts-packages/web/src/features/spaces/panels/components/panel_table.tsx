import { Plus } from 'lucide-react';
import { SpacePanelResponse } from '../types/space-panel-response';
import { PanelName } from './panel_name';
import { PanelQuotas } from './panel_quota';
import { PanelAge } from './panel_ages';
import { PanelGender } from './panel-genders';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Extra } from '@/components/icons';
import { TFunction } from 'i18next';

export type PanelTableProps = {
  panels: SpacePanelResponse[];
  t: TFunction<'SpacePanelEditor', undefined>;
  canEdit: boolean;
  onadd: () => void;
  handleDeletePanel?: (index: number) => void;
  openAgePopup?: (index: number) => void;
  openGenderPopup?: (index: number) => void;
  handleUpdateName?: (index: number, name: string) => void;
  handleUpdateQuotas?: (index: number, quotas: number) => void;
};

export function PanelTable({
  panels,
  t,
  canEdit,
  onadd,
  handleDeletePanel,
  openAgePopup,
  openGenderPopup,
  handleUpdateName,
  handleUpdateQuotas,
}: PanelTableProps) {
  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <colgroup>
          <col className="w-[20%]" />
          <col className="w-[40%]" />
          <col className="w-[20%]" />
          <col className="w-[15%]" />
          <col className="w-[5%]" />
        </colgroup>
        <thead>
          <tr className="bg-card-bg-secondary dark:bg-gray-800 dark:border-gray-700">
            <th className="p-3 font-semibold text-left">{t('panel_name')}</th>
            <th className="p-3 font-semibold text-left">{t('age')}</th>
            <th className="p-3 font-semibold text-left">{t('gender')}</th>
            <th className="p-3 font-semibold text-left">{t('quotas')}</th>
            <th className="p-3 font-semibold text-left">
              {canEdit && (
                <div
                  className="cursor-pointer w-6 h-6 bg-neutral-300 rounded-md"
                  onClick={onadd}
                >
                  <Plus className="w-fit h-fit [&>path]:stroke-gray-700" />
                </div>
              )}
            </th>
          </tr>
        </thead>

        <tbody>
          {panels.map((panel, index) => (
            <tr
              key={panel.pk}
              className="bg-card-bg-secondary/80 border-t border-neutral-500 dark:border-gray-700"
            >
              <td className="p-3">
                <PanelName
                  t={t}
                  canEdit={canEdit}
                  name={panel.name}
                  setName={(name: string) => {
                    handleUpdateName?.(index, name);
                  }}
                />
              </td>
              <td
                className={`p-3 ${canEdit ? 'cursor-pointer' : ''}`}
                onClick={() => {
                  if (canEdit) {
                    openAgePopup?.(index);
                  }
                }}
              >
                <PanelAge t={t} attributes={panel.attributes} />
              </td>
              <td
                className={`p-3 ${canEdit ? 'cursor-pointer' : ''}`}
                onClick={() => {
                  if (canEdit) {
                    openGenderPopup?.(index);
                  }
                }}
              >
                <PanelGender t={t} attributes={panel.attributes} />
              </td>
              <td className="p-3">
                <PanelQuotas
                  quotas={panel.quotas}
                  canEdit={canEdit}
                  setQuotas={(quotas: number) => {
                    handleUpdateQuotas?.(index, quotas);
                  }}
                />
              </td>
              <td className="p-3">
                {canEdit && (
                  <ContextMenu
                    t={t}
                    handleDeletePanel={() => {
                      handleDeletePanel?.(index);
                    }}
                  />
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function ContextMenu({
  t,
  handleDeletePanel,
}: {
  t: TFunction<'SpacePanelEditor', undefined>;
  handleDeletePanel: () => void;
}) {
  return (
    <div>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <button
            className="p-1 hover:bg-hover rounded-full focus:outline-none transition-colors"
            aria-haspopup="true"
            aria-label="Post options for desktop"
          >
            <Extra className="size-6 text-gray-400" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          align="end"
          className="w-40 border-gray-700 transition ease-out duration-100"
        >
          <DropdownMenuItem>
            <button
              aria-label="Delete Panel"
              onClick={handleDeletePanel}
              className="flex items-center w-full px-4 max-tablet:justify-start max-tablet:gap-1 max-tablet:hover:bg-transparent max-tablet:px-0 py-2 text-sm text-neutral-700 hover:bg-gray-700 hover:text-white cursor-pointer"
            >
              {t('delete')}
            </button>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
