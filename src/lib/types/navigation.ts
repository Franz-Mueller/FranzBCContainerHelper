export type AppView =
    | "containers"
    | "images"
    | "projects"
    | "repositories";

export const viewTitles: Record<AppView, string> = {
    containers: "Containers",
    images: "Images",
    projects: "Projects",
    repositories: "Repositories",
};