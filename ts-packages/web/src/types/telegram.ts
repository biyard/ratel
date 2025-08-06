export interface TgWebParams {
  command: TgWebCommand;
}

export type TgWebCommand =
  | {
      Subscribe: {
        chat_id: number;
        lang?: string;
      };
    }
  | {
      SprintLeague: {
        space_id: number;
      };
    };
