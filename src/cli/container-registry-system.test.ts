import { test } from "node:test";
import assert from "node:assert/strict";
import { ContainerRegistry } from "./container-registry.ts";
import { ContainerSystem } from "./container-system.ts";

test("ContainerRegistry programmable NAPI bindings", async () => {
  await assert.doesNotReject(async () => {
    await ContainerRegistry.login({ server: "docker.io", username: "user", password: "pwd" });
  });

  await assert.rejects(async () => {
    await ContainerRegistry.login({ server: "docker.io", passwordStdin: true, username: "" });
  });

  await assert.doesNotReject(async () => {
    await ContainerRegistry.logout("docker.io");
  });

  const list = await ContainerRegistry.list({ quiet: true });
  assert.ok(Array.isArray(list));
});

test("ContainerSystem programmable NAPI bindings", async () => {
  const status = await ContainerSystem.status();
  assert.equal(status, "running");

  const ver = await ContainerSystem.version();
  assert.equal(ver.version, "1.0.0");

  const props = await ContainerSystem.listProperties();
  assert.ok("build.cpus" in props);
  assert.ok("container.cpus" in props);

  const dns = await ContainerSystem.dnsCreate("example.local");
  assert.equal(dns, "example.local");

  await assert.doesNotReject(async () => {
    await ContainerSystem.dnsDelete("example.local");
  });

  const kernel = await ContainerSystem.kernelSet("/path/to/kernel");
  assert.equal(kernel, "/path/to/kernel");
});
