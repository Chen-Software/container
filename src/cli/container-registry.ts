import native from "../../build/index.js";

export class ContainerRegistryCliHandler {
  private registryCli: InstanceType<typeof native.ContainerRegistryCli>;

  constructor() {
    this.registryCli = new native.ContainerRegistryCli();
  }

  async login(server: string, username?: string, password?: string): Promise<string> {
    return this.registryCli.login(server, username, password);
  }

  async logout(server: string): Promise<string> {
    return this.registryCli.logout(server);
  }

  async list(): Promise<Record<string, string>[]> {
    return this.registryCli.list();
  }
}

export const containerRegistryHandler = new ContainerRegistryCliHandler();

const ContainerRef = native.Container;
if (ContainerRef) {
  (ContainerRef as unknown as { registry: typeof containerRegistryHandler }).registry = containerRegistryHandler;
}

declare module "../lib.ts" {
  namespace Container {
    const registry: ContainerRegistryCliHandler;
  }
}

export default ContainerRegistryCliHandler;
