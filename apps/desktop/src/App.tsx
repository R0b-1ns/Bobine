import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { ClipList } from "./library/ClipList";
import type { ClipSummary, Volume } from "./library/types";

export function App() {
  const [clips, setClips] = useState<ClipSummary[]>([]);

  useEffect(() => {
    invoke<ClipSummary[]>("list_clips")
      .then(setClips)
      .catch(() => setClips([]));
    invoke<Volume[]>("list_volumes").catch(() => undefined);
  }, []);

  return (
    <div>
      <aside>
        <div>Bobine</div>
        <div>Clone</div>
        <h2>Recent</h2>
      </aside>
      <main>
        <input aria-label="Search" placeholder="Search" />
        <ClipList clips={clips} />
      </main>
    </div>
  );
}
