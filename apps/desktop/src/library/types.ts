export type ClipKind = "video" | "photo";

export type ClipLocation = {
  volumeId: string;
  volumeName: string;
  path: string;
  connected: boolean;
};

export type ClipSummary = {
  id: string;
  kind: ClipKind;
  originalFilename: string;
  projectId: string;
  projectName: string;
  durationMs: number | null;
  cameraType: string | null;
  colorSpace: string | null;
  locations: ClipLocation[];
};

export type Volume = {
  id: string;
  name: string;
  kind: "internal" | "external" | "nas" | "card";
  stableId: string;
  lastPath: string | null;
  connected: boolean;
};
