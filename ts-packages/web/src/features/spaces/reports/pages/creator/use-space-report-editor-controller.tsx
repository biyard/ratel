import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router';
import { getOption } from '@/features/spaces/polls/hooks/use-topic';
import { useUpdateContentsMutation } from '@/features/spaces/polls/hooks/use-update-contents-mutation';
import { SpaceAnalyze } from '@/features/spaces/polls/types/space-analyze';
import { route } from '@/route';

export interface SpaceReportEditorController {
  analyze: SpaceAnalyze | undefined;
  isLoading: boolean;
  handleUpdateHtmlContents: (htmlContents: string) => void;
  handleDownload: () => void;
}

export function useSpaceReportEditorController(
  spacePk: string,
): SpaceReportEditorController {
  const { data: analyze, isLoading } = useQuery(getOption(spacePk));
  const updateContentsMutation = useUpdateContentsMutation();
  const navigate = useNavigate();

  const handleUpdateHtmlContents = (htmlContents: string) => {
    updateContentsMutation.mutate({
      spacePk,
      htmlContents,
    });
  };

  const handleDownload = () => {
    const analyzePk = String(analyze?.sk ?? '');
    navigate(
      `${route.spaceReportViewer(spacePk)}?analyze_pk=${encodeURIComponent(
        analyzePk,
      )}`,
    );
  };

  return {
    analyze,
    isLoading,
    handleUpdateHtmlContents,
    handleDownload,
  };
}
