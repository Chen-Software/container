import native from "../../build/index.js";
import type { Container } from "../lib.ts";

export class ContainerSystemCliHandler {
  private systemCli: InstanceType<typeof native.ContainerSystemCli>;

  constructor() {
    this.systemCli = new native.ContainerSystemCli();
  }

  async start(): Promise<string> {
    return this.systemCli.start();
  }

  async stop(): Promise<string> {
    return this.systemCli.stop();
  }

  async status(): Promise<string> {
    return this.systemCli.status();
  }

  async version(): Promise<Record<string, string>> {
    return this.systemCli.version();
  }

  async df(): Promise<Record<string, string>> {
    return this.systemCli.df();
  }

  async logs(follow?: boolean): Promise<string[]> {
    return this.systemCli.logs(follow);
  }

  async listProperties(): Promise<Record<string, string>> {
    return this.systemCli.listProperties();
  }

  async dnsCreate(domain: string, ip?: string): Promise<string> {
    return this.systemCli.dnsCreate(domain, ip);
  }

  async dnsList(): Promise<Record<string, string>[]> {
    return this.systemCli.dnsList();
  }

  async dnsDelete(domain: string): Promise<string> {
    return this.systemCli.dnsDelete(domain);
  }

  async kernelSet(path: string): Promise<string> {
    return this.systemCli.kernelSet(path);
  }
}

export const containerSystemHandler = new ContainerSystemCliHandler();

const ContainerRef = native.Container;
if (ContainerRef) {
  (ContainerRef as unknown as { system: typeof containerSystemHandler }).system = containerSystemHandler;
}

declare module "../lib.ts" {
  namespace Container {
    const system: ContainerSystemCliHandler;
  }
}

export default ContainerSystemCliHandler;
