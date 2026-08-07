<script lang="ts">
  import AppSidebar from "$lib/components/layout/AppSidebar.svelte";
  import ContainersPage from "$lib/features/containers/ContainersPage.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";

  import { viewTitles, type AppView } from "$lib/types/navigation";

  let activeView = $state<AppView>("containers");

  function navigateTo(view: AppView): void {
    activeView = view;
  }
</script>

<Sidebar.Provider>
  <AppSidebar {activeView} onNavigate={navigateTo} />

  <Sidebar.Inset>
    <header class="flex h-14 shrink-0 items-center gap-3 border-b px-4">
      <Sidebar.Trigger />

      <div class="h-4 w-px bg-border"></div>

      <span class="font-medium">
        {viewTitles[activeView]}
      </span>
    </header>

    <main class="flex flex-1 flex-col p-4 md:p-6">
      {#if activeView === "containers"}
        <ContainersPage />
      {:else}
        <section
          class="flex min-h-72 flex-col items-center justify-center rounded-lg border border-dashed"
        >
          <h1 class="text-xl font-semibold">
            {viewTitles[activeView]}
          </h1>

          <p class="mt-2 text-sm text-muted-foreground">
            Diese Ansicht wird später ergänzt.
          </p>
        </section>
      {/if}
    </main>
  </Sidebar.Inset>
</Sidebar.Provider>
