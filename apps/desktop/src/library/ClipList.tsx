import type { ClipLocation, ClipSummary } from "./types";

type ClipListProps = {
  clips: ClipSummary[];
};

export function clipStatus(locations: ClipLocation[]): "offline" | "connected" {
  if (locations.length > 0 && locations.every((location) => !location.connected)) {
    return "offline";
  }
  return "connected";
}

export function ClipList({ clips }: ClipListProps) {
  return (
    <ul>
      {clips.map((clip) => (
        <li key={clip.id}>
          <span>{clip.originalFilename}</span>
          <span>{clip.projectName}</span>
          <span>{clipStatus(clip.locations)}</span>
        </li>
      ))}
    </ul>
  );
}
