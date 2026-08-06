<script lang="ts">
    import { Badge } from "$lib/components/ui/badge/index.js";
    import * as Table from "$lib/components/ui/table/index.js";

    import type { DockerContainer, DockerContainerState } from "./container";

    type Props = {
        containers: DockerContainer[];
    };

    let { containers }: Props = $props();

    const statusVariants: Record<
        DockerContainerState,
        "default" | "secondary" | "destructive" | "outline"
    > = {
        created: "outline",
        running: "default",
        paused: "secondary",
        restarting: "outline",
        removing: "outline",
        exited: "destructive",
        dead: "destructive",
        unknown: "secondary",
    };

    const statusLabels: Record<DockerContainerState, string> = {
        created: "Created",
        running: "Running",
        paused: "Paused",
        restarting: "Restarting",
        removing: "Removing",
        exited: "Exited",
        dead: "Dead",
        unknown: "Unknown",
    };

    const dateFormatter = new Intl.DateTimeFormat("de-DE", {
        dateStyle: "medium",
        timeStyle: "short",
    });

    function formatCreatedAt(value: string): string {
        const date = new Date(value);

        if (Number.isNaN(date.getTime())) {
            return value;
        }

        return dateFormatter.format(date);
    }
</script>

<div class="overflow-hidden rounded-lg border">
    <Table.Root>
        <Table.Header>
            <Table.Row>
                <Table.Head>Name</Table.Head>
                <Table.Head>Image</Table.Head>
                <Table.Head>Status</Table.Head>
                <Table.Head>Ports</Table.Head>
                <Table.Head>Created</Table.Head>
                <Table.Head>ID</Table.Head>
            </Table.Row>
        </Table.Header>

        <Table.Body>
            {#each containers as container (container.id)}
                <Table.Row>
                    <Table.Cell class="font-medium">
                        {container.name}
                    </Table.Cell>

                    <Table.Cell>
                        <span
                            class="block max-w-64 truncate font-mono text-xs"
                            title={container.image}
                        >
                            {container.image}
                        </span>
                    </Table.Cell>

                    <Table.Cell>
                        <div class="flex flex-col items-start gap-1">
                            <Badge variant={statusVariants[container.state]}>
                                {statusLabels[container.state]}
                            </Badge>

                            <span class="text-xs text-muted-foreground">
                                {container.status}
                            </span>
                        </div>
                    </Table.Cell>

                    <Table.Cell>
                        {#if container.ports.length > 0}
                            <div class="flex flex-col gap-1">
                                {#each container.ports as port (port)}
                                    <span class="font-mono text-xs">
                                        {port}
                                    </span>
                                {/each}
                            </div>
                        {:else}
                            <span class="text-muted-foreground">—</span>
                        {/if}
                    </Table.Cell>

                    <Table.Cell class="whitespace-nowrap">
                        {formatCreatedAt(container.createdAt)}
                    </Table.Cell>

                    <Table.Cell>
                        <span
                            class="font-mono text-xs text-muted-foreground"
                            title={container.id}
                        >
                            {container.id.slice(0, 12)}
                        </span>
                    </Table.Cell>
                </Table.Row>
            {:else}
                <Table.Row>
                    <Table.Cell
                        colspan={6}
                        class="h-32 text-center text-muted-foreground"
                    >
                        Keine Container vorhanden.
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
