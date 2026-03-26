export interface DockerService {
  name: string;
  status: string;
  ports: string;
}

export interface DockerComposeStatus {
  hasCompose: boolean;
  services: DockerService[];
  composeFile: string;
}
