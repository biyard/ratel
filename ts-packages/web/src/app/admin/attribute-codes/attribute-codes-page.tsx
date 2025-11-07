import { useState } from 'react';
import { useAttributeCodesPageController } from './use-attribute-codes-page-controller';
import { useAttributeCodesI18n } from './attribute-codes-page-i18n';
import type { CreateAttributeCodeRequest } from '@/features/did/dto/create-attribute-code-request';
import type { Gender } from '@/features/did/dto/attribute-code-response';

export function AttributeCodesPage() {
  const ctrl = useAttributeCodesPageController();
  const i18n = useAttributeCodesI18n();

  if (ctrl.isLoading) {
    return (
      <div className="p-6 mx-auto w-full max-w-desktop">
        <div className="py-8 text-center">{i18n.loading}</div>
      </div>
    );
  }

  if (ctrl.error) {
    return (
      <div className="p-6 mx-auto w-full max-w-desktop">
        <div className="py-8 text-center text-red-500">
          {i18n.loadError}: {ctrl.error.message}
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 mx-auto w-full max-w-desktop">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">{i18n.title}</h1>
        <button
          onClick={ctrl.openCreateForm}
          className="py-2 px-4 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
        >
          {i18n.createNew}
        </button>
      </div>

      <div className="bg-white rounded-lg shadow dark:bg-gray-800">
        {ctrl.attributeCodes.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{i18n.noData}</div>
        ) : (
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-900">
              <tr>
                <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-300">
                  {i18n.code}
                </th>
                <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-300">
                  {i18n.attributes}
                </th>
                <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-300">
                  {i18n.createdAt}
                </th>
                <th className="py-3 px-6 text-xs font-medium tracking-wider text-right text-gray-700 uppercase dark:text-gray-300">
                  {i18n.actions}
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
              {ctrl.attributeCodes.map((code) => (
                <tr
                  key={code.pk}
                  className="hover:bg-gray-50 dark:hover:bg-gray-700"
                >
                  <td className="py-4 px-6 font-mono text-sm text-gray-900 whitespace-nowrap dark:text-gray-100">
                    {code.code}
                  </td>
                  <td className="py-4 px-6 text-sm text-gray-700 dark:text-gray-300">
                    {code.getDisplayAttributes()}
                  </td>
                  <td className="py-4 px-6 text-sm text-gray-700 whitespace-nowrap dark:text-gray-300">
                    {code.getFormattedDate()}
                  </td>
                  <td className="py-4 px-6 text-sm text-right whitespace-nowrap">
                    <button
                      onClick={() => ctrl.openDeleteConfirm(code)}
                      className="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                    >
                      {i18n.delete}
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {ctrl.isFormOpen && (
        <CreateAttributeCodeForm
          onSubmit={ctrl.handleCreateAttributeCode}
          onCancel={ctrl.closeForm}
          isSubmitting={ctrl.isSubmitting}
          i18n={i18n}
        />
      )}

      {ctrl.deleteConfirmCode && (
        <DeleteConfirmDialog
          code={ctrl.deleteConfirmCode}
          onConfirm={ctrl.handleDeleteAttributeCode}
          onCancel={ctrl.closeDeleteConfirm}
          isDeleting={ctrl.isSubmitting}
          i18n={i18n}
        />
      )}
    </div>
  );
}

function CreateAttributeCodeForm({
  onSubmit,
  onCancel,
  isSubmitting,
  i18n,
}: {
  onSubmit: (request: CreateAttributeCodeRequest) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
  i18n: ReturnType<typeof useAttributeCodesI18n>;
}) {
  const [birthDate, setBirthDate] = useState('');
  const [gender, setGender] = useState<Gender | ''>('');
  const [university, setUniversity] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    // Validate at least one field is filled
    if (!birthDate && !gender && !university) {
      setError(i18n.atLeastOne);
      return;
    }

    const request: CreateAttributeCodeRequest = {};
    if (birthDate) request.birth_date = birthDate;
    if (gender) request.gender = gender;
    if (university) request.university = university;

    try {
      await onSubmit(request);
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="z-50 fixed inset-0 flex justify-center items-center bg-black bg-opacity-50">
      <div className="p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-bold">{i18n.createTitle}</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.birthDate} {i18n.optional}
            </label>
            <input
              type="text"
              value={birthDate}
              onChange={(e) => setBirthDate(e.target.value)}
              placeholder={i18n.birthDatePlaceholder}
              className="px-3 py-2 w-full rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              pattern="[0-9]{8}"
              maxLength={8}
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.gender} {i18n.optional}
            </label>
            <select
              value={gender}
              onChange={(e) => setGender(e.target.value as Gender | '')}
              className="px-3 py-2 w-full rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            >
              <option value="">-</option>
              <option value="male">{i18n.male}</option>
              <option value="female">{i18n.female}</option>
            </select>
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.university} {i18n.optional}
            </label>
            <input
              type="text"
              value={university}
              onChange={(e) => setUniversity(e.target.value)}
              placeholder={i18n.universityPlaceholder}
              className="px-3 py-2 w-full rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            />
          </div>

          {error && <div className="mb-4 text-sm text-red-500">{error}</div>}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={onCancel}
              className="py-2 px-4 text-gray-700 bg-gray-200 rounded hover:bg-gray-300 dark:bg-gray-600 dark:text-gray-200 dark:hover:bg-gray-500"
              disabled={isSubmitting}
            >
              {i18n.cancel}
            </button>
            <button
              type="submit"
              className="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600 disabled:opacity-50"
              disabled={isSubmitting}
            >
              {isSubmitting ? i18n.submitting : i18n.submit}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

function DeleteConfirmDialog({
  code,
  onConfirm,
  onCancel,
  isDeleting,
  i18n,
}: {
  code: { code: string };
  onConfirm: () => Promise<void>;
  onCancel: () => void;
  isDeleting: boolean;
  i18n: ReturnType<typeof useAttributeCodesI18n>;
}) {
  return (
    <div className="z-50 fixed inset-0 flex justify-center items-center bg-black bg-opacity-50">
      <div className="p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-bold">{i18n.deleteConfirm}</h2>
        <p className="mb-6 text-gray-700 dark:text-gray-300">
          Code: <span className="font-mono font-bold">{code.code}</span>
        </p>
        <div className="flex gap-2 justify-end">
          <button
            onClick={onCancel}
            className="py-2 px-4 text-gray-700 bg-gray-200 rounded hover:bg-gray-300 dark:bg-gray-600 dark:text-gray-200 dark:hover:bg-gray-500"
            disabled={isDeleting}
          >
            {i18n.cancel}
          </button>
          <button
            onClick={onConfirm}
            className="py-2 px-4 text-white bg-red-500 rounded hover:bg-red-600 disabled:opacity-50"
            disabled={isDeleting}
          >
            {i18n.delete}
          </button>
        </div>
      </div>
    </div>
  );
}
