import { Container, type ComposeHandle } from "../lib.ts";

export class ContainerComposeCliHandler {
  private composeHandle: ComposeHandle;

  constructor(runtime?: Container) {
    const rt = runtime ?? Container.withDefaultConfig();
    this.composeHandle = rt.compose;
  }

  async up(detach?: boolean, build?: boolean): Promise<void> {
    return this.composeHandle.up(detach, build);
  }

  async down(volumes?: boolean): Promise<void> {
    return this.composeHandle.down(volumes);
  }

  async start(): Promise<void> {
    return this.composeHandle.start();
  }

  async stop(): Promise<void> {
    return this.composeHandle.stop();
  }

  async restart(): Promise<void> {
    return this.composeHandle.restart();
  }

  async create(): Promise<void> {
    return this.composeHandle.create();
  }

  async kill(signal?: string): Promise<void> {
    return this.composeHandle.kill(signal);
  }

  async rm(force?: boolean): Promise<void> {
    return this.composeHandle.rm(force);
  }

  async ps(): Promise<string[]> {
    return this.composeHandle.ps();
  }

  async ls(): Promise<string[]> {
    return this.composeHandle.ls();
  }

  async logs(follow?: boolean): Promise<string[]> {
    return this.composeHandle.logs(follow);
  }

  async top(): Promise<string[]> {
    return this.composeHandle.top();
  }

  async port(service: string, privatePort: number): Promise<string> {
    return this.composeHandle.port(service, privatePort);
  }

  async events(): Promise<string[]> {
    return this.composeHandle.events();
  }

  async config(): Promise<string> {
    return this.composeHandle.config();
  }

  async build(): Promise<void> {
    return this.composeHandle.build();
  }

  async run(service: string, command?: string[]): Promise<number> {
    return this.composeHandle.run(service, command);
  }

  async exec(service: string, command: string[]): Promise<number> {
    return this.composeHandle.exec(service, command);
  }

  async watch(): Promise<void> {
    return this.composeHandle.watch();
  }

  async pull(): Promise<void> {
    return this.composeHandle.pull();
  }

  async push(): Promise<void> {
    return this.composeHandle.push();
  }

  async serve(): Promise<void> {
    return this.composeHandle.serve();
  }

  async version(): Promise<string> {
    return this.composeHandle.version();
  }
}

export default ContainerComposeCliHandler;
