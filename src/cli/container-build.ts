import { Container, type BuildOptions } from "../lib.ts";

export interface ContainerBuildCommandOptions extends BuildOptions {
  contextDir?: string;
}

export async function handleContainerBuild(
  contextDir: string = ".",
  options: ContainerBuildCommandOptions = {},
): Promise<string> {
  const runtime = Container.withDefaultConfig();
  return runtime.images.build(contextDir, options);
}

export default handleContainerBuild;
