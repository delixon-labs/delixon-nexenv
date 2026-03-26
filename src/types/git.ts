export interface GitCommit {
  hash: string;
  message: string;
  author: string;
  date: string;
}

export interface GitStatus {
  branch: string;
  isClean: boolean;
  modifiedFiles: number;
  untrackedFiles: number;
  ahead: number;
  behind: number;
  hasRemote: boolean;
  lastCommit: GitCommit | null;
}
