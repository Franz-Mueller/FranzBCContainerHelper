<script lang="ts">
    import BoxesIcon from "@lucide/svelte/icons/boxes";
    import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
    import GitForkIcon from "@lucide/svelte/icons/git-fork";
    import LayersIcon from "@lucide/svelte/icons/layers";
    import PackageIcon from "@lucide/svelte/icons/package";

    import * as Sidebar from "$lib/components/ui/sidebar/index.js";

    import type { AppView } from "$lib/types/navigation";

    type Props = {
        activeView: AppView;
        onNavigate: (view: AppView) => void;
    };

    let { activeView, onNavigate }: Props = $props();

    const navigationItems = [
        {
            id: "containers",
            title: "Containers",
            icon: BoxesIcon,
        },
        {
            id: "images",
            title: "Images",
            icon: LayersIcon,
        },
        {
            id: "projects",
            title: "Projects",
            icon: FolderKanbanIcon,
        },
        {
            id: "repositories",
            title: "Repositories",
            icon: GitForkIcon,
        },
    ] as const;
</script>

<Sidebar.Root collapsible="icon">
    <Sidebar.Header>
        <div class="flex h-12 items-center gap-2 px-2">
            <div
                class="flex size-8 shrink-0 items-center justify-center rounded-md bg-primary text-primary-foreground"
            >
                <PackageIcon class="size-4" />
            </div>

            <div class="min-w-0">
                <div class="truncate text-sm font-semibold">
                    Franz BC Helper
                </div>

                <div class="truncate text-xs text-muted-foreground">
                    Container Management
                </div>
            </div>
        </div>
    </Sidebar.Header>

    <Sidebar.Separator />

    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupLabel>Docker</Sidebar.GroupLabel>

            <Sidebar.GroupContent>
                <Sidebar.Menu>
                    {#each navigationItems as item (item.id)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton
                                isActive={activeView === item.id}
                                onclick={() => onNavigate(item.id)}
                                title={item.title}
                            >
                                <item.icon />
                                <span>{item.title}</span>
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>

    <Sidebar.Footer>
        <div class="px-2 py-2 text-xs text-muted-foreground">Development</div>
    </Sidebar.Footer>

    <Sidebar.Rail />
</Sidebar.Root>
