import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { ClipList } from "./library/ClipList";
import type { ClipSummary, Volume } from "./library/types";

type AppProps = {
  loadClips?: () => Promise<ClipSummary[]>;
};

function defaultLoadClips(): Promise<ClipSummary[]> {
  return invoke<ClipSummary[]>("list_clips");
}

export function App({ loadClips = defaultLoadClips }: AppProps) {
  const [clips, setClips] = useState<ClipSummary[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    loadClips()
      .then((next) => {
        if (cancelled) {
          return;
        }
        setError(null);
        setClips(next);
      })
      .catch(() => {
        if (!cancelled) {
          setError("Could not load the library");
        }
      });
    invoke<Volume[]>("list_volumes").catch(() => undefined);
    return () => {
      cancelled = true;
    };
  }, [loadClips]);

  return (
    <div>
      <aside>
        <div>Bobine</div>
        <div>Clone</div>
        <h2>Recent</h2>
      </aside>
      <main>
        <input aria-label="Search" placeholder="Search" />
        {error ? <p role="alert">{error}</p> : <ClipList clips={clips} />}
      </main>
    </div>
  );
}
