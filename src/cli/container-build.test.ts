import { expect, test } from "vite-plus/test";
import { ContainerBuildCli } from "../lib.ts";
import { Container, handleContainerBuild } from "./container-build.ts";

test("ContainerBuildCli NAPI bindings work directly", () => {
  const cli = new ContainerBuildCli();
  expect(cli).toBeDefined();
  const result = cli.build("./my-app", { dockerfile: "Dockerfile" });
  expect(result).toBe("built image from ./my-app");
});

test("handleContainerBuild function works", async () => {
  const result = await handleContainerBuild("./my-app");
  expect(result).toBe("built image from ./my-app");
});

test("Container.build function works under global declaration", async () => {
  const fn = (Container as unknown as { build: (dir: string) => Promise<string> }).build;
  expect(fn).toBeDefined();
  const result = await fn("./my-app");
  expect(result).toBe("built image from ./my-app");
});
