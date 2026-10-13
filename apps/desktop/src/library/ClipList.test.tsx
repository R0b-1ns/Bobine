import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { ClipList } from "./ClipList";

it("shows offline only when every location is disconnected", () => {
  render(
    <ClipList
      clips={[
        {
          id: "c1",
          kind: "video",
          originalFilename: "A001_C001.mov",
          projectId: "p1",
          projectName: "Martin — 5 shorts",
          durationMs: null,
          cameraType: null,
          colorSpace: null,
          locations: [
            { volumeId: "v1", volumeName: "Backup HDD", path: "A001_C001.mov", connected: false },
          ],
        },
        {
          id: "c2",
          kind: "video",
          originalFilename: "street_dusk.mov",
          projectId: "p1",
          projectName: "Martin — 5 shorts",
          durationMs: null,
          cameraType: null,
          colorSpace: null,
          locations: [
            { volumeId: "v2", volumeName: "SSD 2", path: "broll/street_dusk.mov", connected: true },
          ],
        },
      ]}
    />,
  );
  expect(screen.getByText("A001_C001.mov").closest("li")).toHaveTextContent(/offline/i);
  expect(screen.getByText("street_dusk.mov").closest("li")).toHaveTextContent(/connected/i);
  expect(screen.getByText("street_dusk.mov").closest("li")).not.toHaveTextContent(/offline/i);
});
