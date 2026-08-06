import type { DockerContainer } from "./container";

export const mockContainers: DockerContainer[] = [
    {
        id: "af31d10490b946eab23341ae",
        name: "bc25-dev",
        image: "businesscentral:sandbox-25",
        state: "running",
        status: "Up 2 hours",
        ports: ["8080:80", "7046:7046", "7048:7048"],
        createdAt: "2026-08-06T10:32:00+02:00",
    },
    {
        id: "bce84e2ec73a40f6a85710ea",
        name: "bc24-test",
        image: "businesscentral:sandbox-24",
        state: "exited",
        status: "Exited (0) 45 minutes ago",
        ports: ["8081:80"],
        createdAt: "2026-08-05T16:18:00+02:00",
    },
    {
        id: "7cad23e394a745e6ab199c81",
        name: "bc-clean",
        image: "businesscentral:sandbox-25",
        state: "paused",
        status: "Paused",
        ports: [],
        createdAt: "2026-08-04T09:05:00+02:00",
    },
];