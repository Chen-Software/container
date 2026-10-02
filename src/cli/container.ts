import { Container, type ContainerOptions, type ContainerInfo } from "../lib.ts";
import { handleContainerBuild, type ContainerBuildCommandOptions } from "./container-build.ts";

export class ContainerCliHandler {
  private runtime: Container;

  constructor(runtime?: Container) {
    this.runtime = runtime ?? Container.withDefaultConfig();
  }

  async build(contextDir: string = ".", options: ContainerBuildCommandOptions = {}): Promise<string> {
    return handleContainerBuild(contextDir, options);
  }

  async run(options: ContainerOptions, name?: string): Promise<Container> {
    return this.runtime.run(options, name);
  }

  async create(options: ContainerOptions, name?: string): Promise<Container> {
    return this.runtime.create(options, name);
  }

  async start(container: Container, attach?: boolean, interactive?: boolean): Promise<number> {
    return container.start(attach, interactive);
  }

  async stop(container: Container, signal?: string, time?: number): Promise<void> {
    return container.stop(signal, time);
  }

  async exec(
    container: Container,
    cmd: string[],
    env?: Record<string, string>,
    cwd?: string,
    user?: string,
    detach?: boolean,
    interactive?: boolean,
    tty?: boolean,
  ): Promise<number> {
    return container.exec(cmd, env, cwd, user, detach, interactive, tty);
  }

  async inspect(container: Container): Promise<ContainerInfo> {
    return container.inspect();
  }

  async logs(container: Container, follow?: boolean, tail?: number, boot?: boolean): Promise<string[]> {
    return container.logs(follow, tail, boot);
  }

  async stats(container: Container, noStream?: boolean): Promise<Record<string, string>> {
    return container.stats(noStream);
  }

  async clean(): Promise<void> {
    return this.runtime.clean();
  }

  async prune(): Promise<string[]> {
    return this.runtime.prune();
  }

  async copy(container: Container, src: string, dest: string): Promise<void> {
    return container.copy(src, dest);
  }

  async commit(container: Container, reference: string): Promise<string> {
    return container.commit(reference);
  }

  async export(container: Container, outputPath?: string): Promise<Uint8Array> {
    return container.export(outputPath);
  }

  async list(): Promise<ContainerInfo[]> {
    return this.runtime.listInfo();
  }

  async remove(idOrName: string, force?: boolean): Promise<void> {
    return this.runtime.remove(idOrName, force);
  }
}

export default ContainerCliHandler;
