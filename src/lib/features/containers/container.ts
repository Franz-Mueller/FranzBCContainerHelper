export type DockerContainerState =
    | "created"
    | "running"
    | "paused"
    | "restarting"
    | "removing"
    | "exited"
    | "dead"
    | "unknown";

export interface DockerContainer {
    id: string;
    name: string;
    image: string;

    /**
     * Maschinenlesbarer Zustand.
     * Wird für Farben, Filter und Aktionen verwendet.
     */
    state: DockerContainerState;

    /**
     * Menschenlesbarer Statustext, beispielsweise:
     * "Up 2 hours" oder "Exited (0) 5 minutes ago".
     */
    status: string;

    /**
     * Bereits formatierte Portzuordnungen.
     * Beispiel: ["8080:80", "7048:7048"]
     */
    ports: string[];

    /**
     * ISO-8601-Datum als String.
     * Beispiel: "2026-08-06T10:30:00+02:00"
     */
    createdAt: string;
}