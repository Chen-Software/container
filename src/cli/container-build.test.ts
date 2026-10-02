import { expect, test } from "vite-plus/test";
import {
  Container,
  ContainerBuildCli,
  ContainerRegistryCli,
  ContainerSystemCli,
  ContainerSystemConfigLoader,
} from "../lib.ts";
import { handleContainerBuild } from "./container-build.ts";
import { ContainerComposeCliHandler } from "./container-compose.ts";
import { ContainerRegistryCliHandler } from "./container-registry.ts";
import { ContainerSystemCliHandler } from "./container-system.ts";

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

test("Container.build function works under declaration merging", async () => {
  const fn = (Container as unknown as { build: (dir: string) => Promise<string> }).build;
  expect(fn).toBeDefined();
  const result = await fn("./my-app");
  expect(result).toBe("built image from ./my-app");
});

test("Container merged static properties (registry, system, compose) work", async () => {
  const containerObj = Container as unknown as {
    registry: ContainerRegistryCliHandler;
    system: ContainerSystemCliHandler;
    compose: ContainerComposeCliHandler;
  };

  expect(containerObj.registry).toBeDefined();
  expect(await containerObj.registry.login("ghcr.io", "user")).toBe("logged into registry ghcr.io as user");

  expect(containerObj.system).toBeDefined();
  expect(await containerObj.system.status()).toBe("running");

  expect(containerObj.compose).toBeDefined();
  expect(await containerObj.compose.version()).toContain("container-compose");
});

test("ContainerRegistryCli and ContainerRegistryCliHandler work", async () => {
  const cli = new ContainerRegistryCli();
  expect(cli.login("docker.io", "user", "pass")).toBe("logged into registry docker.io as user");
  expect(cli.logout("docker.io")).toBe("logged out from registry docker.io");

  const handler = new ContainerRegistryCliHandler();
  expect(await handler.login("docker.io", "admin")).toBe("logged into registry docker.io as admin");
});

test("ContainerSystemCli and ContainerSystemCliHandler work", async () => {
  const cli = new ContainerSystemCli();
  expect(cli.start()).toBe("system started");
  expect(cli.status()).toBe("running");

  const handler = new ContainerSystemCliHandler();
  expect(await handler.status()).toBe("running");
});

test("ContainerSystemConfigLoader parses TOML config", () => {
  const toml = `
[build]
rosetta = true
cpus = 4
memory = "4096mb"
image = "custom-builder:latest"

[container]
cpus = 8
memory = "2gb"

[registry]
domain = "ghcr.io"
`;
  const config = ContainerSystemConfigLoader.parse(toml);
  expect(config.build.cpus).toBe(4);
  expect(config.build.rosetta).toBe(true);
  expect(config.container.cpus).toBe(8);
  expect(config.registry.domain).toBe("ghcr.io");
});
