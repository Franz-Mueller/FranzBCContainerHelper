<script lang="ts">
    import PlusIcon from "@lucide/svelte/icons/plus";
    import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";

    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";

    import ContainerTable from "./ContainerTable.svelte";
    import { mockContainers } from "./mock-containers";

    const containers = mockContainers;

    const runningCount = containers.filter(
        (container) => container.state === "running",
    ).length;

    const inactiveCount = containers.length - runningCount;
</script>

<div class="space-y-6">
    <div
        class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
    >
        <div>
            <h1 class="text-2xl font-semibold tracking-tight">Containers</h1>

            <p class="text-sm text-muted-foreground">
                Docker-Container anzeigen und verwalten.
            </p>
        </div>

        <div class="flex gap-2">
            <Button variant="outline">
                <RefreshCwIcon class="size-4" />
                Aktualisieren
            </Button>

            <Button>
                <PlusIcon class="size-4" />
                Container erstellen
            </Button>
        </div>
    </div>

    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <Card.Root>
            <Card.Header>
                <Card.Description>Container gesamt</Card.Description>
                <Card.Title class="text-3xl">
                    {containers.length}
                </Card.Title>
            </Card.Header>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Description>Aktiv</Card.Description>
                <Card.Title class="text-3xl">
                    {runningCount}
                </Card.Title>
            </Card.Header>
        </Card.Root>

        <Card.Root>
            <Card.Header>
                <Card.Description>Inaktiv</Card.Description>
                <Card.Title class="text-3xl">
                    {inactiveCount}
                </Card.Title>
            </Card.Header>
        </Card.Root>
    </div>

    <ContainerTable {containers} />
</div>
