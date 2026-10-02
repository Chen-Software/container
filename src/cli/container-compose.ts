import native from "../../build/index.js";

export class ContainerComposeCliHandler {
  private composeCli: InstanceType<typeof native.ContainerComposeCli>;

  constructor() {
    this.composeCli = new native.ContainerComposeCli();
  }

  async up(detach?: boolean, build?: boolean): Promise<string> {
    return this.composeCli.up(detach, build);
  }

  async down(volumes?: boolean): Promise<string> {
    return this.composeCli.down(volumes);
  }

  async start(): Promise<string> {
    return this.composeCli.start();
  }

  async stop(): Promise<string> {
    return this.composeCli.stop();
  }

  async restart(): Promise<string> {
    return this.composeCli.restart();
  }

  async ps(): Promise<string[]> {
    return this.composeCli.ps();
  }

  async ls(): Promise<string[]> {
    return this.composeCli.ls();
  }

  async logs(follow?: boolean): Promise<string[]> {
    return this.composeCli.logs(follow);
  }

  async build(): Promise<string> {
    return this.composeCli.build();
  }

  async config(): Promise<string> {
    return this.composeCli.config();
  }

  async run(service: string, command?: string[]): Promise<number> {
    return this.composeCli.run(service, command);
  }

  async exec(service: string, command: string[]): Promise<number> {
    return this.composeCli.exec(service, command);
  }

  async version(): Promise<string> {
    return this.composeCli.version();
  }

  async status(): Promise<Record<string, string>> {
    return this.composeCli.status();
  }
}

export const containerComposeHandler = new ContainerComposeCliHandler();

const ContainerRef = native.Container;
if (ContainerRef) {
  (ContainerRef as unknown as { compose: typeof containerComposeHandler }).compose = containerComposeHandler;
}

declare module "../lib.ts" {
  namespace Container {
    const compose: ContainerComposeCliHandler;
  }
}

export default ContainerComposeCliHandler;
