import { logger } from './logger';

/**
 * Configuration for URL replacement in PDF files
 */
export interface UrlReplacement {
  /** Original URL or URL pattern to find */
  original: string;
  /** New URL to replace with */
  replace: string;
}

/**
 * @example
 * ```typescript
 * // Basic usage
 * const result = await replacePdfLinks({
 *   url: 'https://example.com/document.pdf',
 *   urlReplacements: [
 *     {
 *       original: 'https://old-domain.com',
 *       replace: 'https://new-domain.com'
 *     }
 *   ]
 * });
 * ```
 *
 * @example
 * ```typescript
 * // Usage in React component
 * <button
 *   onClick={async () => {
 *     const result = await replacePdfLinks({
 *       url: 'http://localhost:3000/test.pdf',
 *       urlReplacements: [
 *         {
 *           original: 'https://old-domain-1.com',
 *           replace: 'https://new-domain-1.com'
 *         },
 *         {
 *           original: 'https://old-domain-2.com',
 *           replace: 'https://new-domain-2.com'
 *         }
 *       ],
 *     });
 *   }}
 * >
 *   Process PDF
 * </button>
 * ```
 */

export async function downloadPdfFromUrl({
  url,
  fileName = 'document.pdf',
}: {
  url: string;
  fileName?: string;
}) {
  if (typeof window === 'undefined') {
    throw new Error('This function can only be called in the browser');
  }

  try {
    console.debug('Fetching PDF from:', url);
    const proxyUrl = `/api/proxy-pdf?url=${encodeURIComponent(url)}`;

    const pdfResponse = await fetch(proxyUrl);
    if (!pdfResponse.ok) {
      throw new Error(`Failed to fetch PDF: ${pdfResponse.statusText}`);
    }

    const pdfBlob = await pdfResponse.blob();

    const downloadLink = document.createElement('a');
    downloadLink.href = URL.createObjectURL(pdfBlob);
    downloadLink.download = fileName;
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
    URL.revokeObjectURL(downloadLink.href);

    console.debug('PDF download completed');

    return {
      success: true,
      message: 'PDF successfully downloaded.',
    };
  } catch (error) {
    console.error('Error downloading PDF:', error);
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

export async function replacePdfLinks({
  url,
  fileName = 'document.pdf',
  urlReplacements,
}: {
  url: string;
  fileName?: string;
  urlReplacements: UrlReplacement[];
}) {
  if (typeof window === 'undefined') {
    throw new Error('This function can only be called in the browser');
  }

  try {
    logger.debug('Fetching PDF from:', url);
    const proxyUrl = `/api/proxy-pdf?url=${encodeURIComponent(url)}`;

    const pdfResponse = await fetch(proxyUrl);
    if (!pdfResponse.ok) {
      throw new Error(`Failed to fetch PDF: ${pdfResponse.statusText}`);
    }

    const pdfArrayBuffer = await pdfResponse.arrayBuffer();
    logger.debug('PDF loaded, size:', pdfArrayBuffer.byteLength);

    const pdfUint8Array = new Uint8Array(pdfArrayBuffer);

    let pdfBinaryString = '';
    for (let byteIndex = 0; byteIndex < pdfUint8Array.length; byteIndex++) {
      pdfBinaryString += String.fromCharCode(pdfUint8Array[byteIndex]);
    }
    logger.debug('PDF converted to string, length:', pdfBinaryString.length);

    let totalReplacements = 0;

    urlReplacements.forEach((urlReplacement) => {
      const escapedOriginalUrl = urlReplacement.original.replace(
        /[.*+?^${}()|[\]\\]/g,
        '\\$&',
      );

      const uriPatterns = [
        {
          regex: new RegExp(
            `(/URI\\s*\\([^)]*?)(${escapedOriginalUrl})([^)]*?\\))`,
            'gi',
          ),
          replacement: `$1${urlReplacement.replace}$3`,
        },
        {
          regex: new RegExp(
            `(/A\\s*<<[^>]*?/URI\\s*\\([^)]*?)(${escapedOriginalUrl})([^)]*?\\)[^>]*?>>)`,
            'gi',
          ),
          replacement: `$1${urlReplacement.replace}$3`,
        },
        {
          regex: new RegExp(
            `(<<[^>]*?/S\\s*/URI[^>]*?/URI\\s*\\([^)]*?)(${escapedOriginalUrl})([^)]*?\\)[^>]*?>>)`,
            'gi',
          ),
          replacement: `$1${urlReplacement.replace}$3`,
        },
        {
          regex: new RegExp(`\\b${escapedOriginalUrl}\\b`, 'gi'),
          replacement: urlReplacement.replace,
        },
      ];

      uriPatterns.forEach((uriPattern) => {
        const patternMatches = pdfBinaryString.match(uriPattern.regex);
        if (patternMatches) {
          logger.debug(`Found ${patternMatches.length} matches for pattern`);
          totalReplacements += patternMatches.length;
          pdfBinaryString = pdfBinaryString.replace(
            uriPattern.regex,
            uriPattern.replacement,
          );
        }
      });
    });

    const modifiedPdfArray = new Uint8Array(pdfBinaryString.length);
    for (
      let stringIndex = 0;
      stringIndex < pdfBinaryString.length;
      stringIndex++
    ) {
      modifiedPdfArray[stringIndex] = pdfBinaryString.charCodeAt(stringIndex);
    }

    const pdfBlob = new Blob([modifiedPdfArray], { type: 'application/pdf' });
    const downloadLink = document.createElement('a');
    downloadLink.href = URL.createObjectURL(pdfBlob);
    downloadLink.download = fileName;
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
    URL.revokeObjectURL(downloadLink.href);

    logger.debug('PDF download completed');

    return {
      success: true,
      message: `PDF links successfully replaced. ${totalReplacements} replacements made.`,
      replacements: totalReplacements,
    };
  } catch (processingError) {
    console.error('Error processing PDF:', processingError);
    return {
      success: false,
      error:
        processingError instanceof Error
          ? processingError.message
          : 'Unknown error',
    };
  }
}
