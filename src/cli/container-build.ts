import native from "../../build/index.js";
import type { BuildOptions } from "../lib.ts";

export interface ContainerBuildCommandOptions extends BuildOptions {
  contextDir?: string;
}

export async function handleContainerBuild(
  contextDir: string = ".",
  options: ContainerBuildCommandOptions = {},
): Promise<string> {
  const buildCli = new native.ContainerBuildCli();
  return buildCli.build(contextDir, options);
}

// Attach `build` function under Container.*
const Container = native.Container;
if (Container) {
  (Container as unknown as { build: typeof handleContainerBuild }).build = handleContainerBuild;
}

declare module "../lib.ts" {
  namespace Container {
    function build(contextDir?: string, options?: ContainerBuildCommandOptions): Promise<string>;
  }
}

export { Container };
export default handleContainerBuild;
