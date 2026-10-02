import native from "../../build/index.js";

export interface RegistryLoginParams {
  server: string;
  username?: string;
  password?: string;
  passwordStdin?: boolean;
  scheme?: string;
}

export interface RegistryListParams {
  format?: string;
  quiet?: boolean;
}

export class ContainerRegistryCliHandler {
  private runtime = native.Container.withDefaultConfig();

  get registry() {
    return this.runtime.registry;
  }

  async login(params: RegistryLoginParams) {
    return this.registry.login(
      params.server,
      params.username,
      params.password,
      params.passwordStdin,
      params.scheme,
    );
  }

  async logout(server: string) {
    return this.registry.logout(server);
  }

  async list(params: RegistryListParams = {}) {
    return this.registry.list(params.format, params.quiet);
  }
}

export const containerRegistryCli = new ContainerRegistryCliHandler();

export const ContainerRegistry = {
  login: async (params: RegistryLoginParams) => containerRegistryCli.login(params),
  logout: async (server: string) => containerRegistryCli.logout(server),
  list: async (params?: RegistryListParams) => containerRegistryCli.list(params),
};
