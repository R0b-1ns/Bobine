import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { App } from "./App";

it("shows an error instead of an empty library when listing fails", async () => {
  render(<App loadClips={() => Promise.reject(new Error("db down"))} />);
  expect(await screen.findByRole("alert")).toHaveTextContent(/could not load/i);
  expect(screen.queryByRole("list")).not.toBeInTheDocument();
});
