import native from "../../build/index.js";

export class ContainerSystemCliHandler {
  private runtime = native.Container.withDefaultConfig();

  get system() {
    return this.runtime.system;
  }

  async status() {
    return this.system.status();
  }

  async version() {
    return this.system.version();
  }

  async df() {
    return this.system.df();
  }

  async logs(follow?: boolean, last?: string) {
    return this.system.logs(follow, last);
  }

  async listProperties() {
    return this.system.listProperties();
  }

  async dnsCreate(domain: string, ip?: string) {
    return this.system.dnsCreate(domain, ip);
  }

  async dnsList() {
    return this.system.dnsList();
  }

  async dnsDelete(domain: string) {
    return this.system.dnsDelete(domain);
  }

  async kernelSet(path: string) {
    return this.system.kernelSet(path);
  }
}

export const containerSystemCli = new ContainerSystemCliHandler();

export const ContainerSystem = {
  status: async () => containerSystemCli.status(),
  version: async () => containerSystemCli.version(),
  df: async () => containerSystemCli.df(),
  logs: async (follow?: boolean, last?: string) => containerSystemCli.logs(follow, last),
  listProperties: async () => containerSystemCli.listProperties(),
  dnsCreate: async (domain: string, ip?: string) => containerSystemCli.dnsCreate(domain, ip),
  dnsList: async () => containerSystemCli.dnsList(),
  dnsDelete: async (domain: string) => containerSystemCli.dnsDelete(domain),
  kernelSet: async (path: string) => containerSystemCli.kernelSet(path),
};
