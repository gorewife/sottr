<script lang="ts">
  let { open, onSubmit } = $props<{
    open: boolean;
    onSubmit: (link: string) => void;
  }>();

  let link = $state("");
</script>

{#if open}
  <div class="dialog">
    <h2>Add link</h2>

    <input
      type="url"
      bind:value={link}
    />

    <div class="actions">
      <button onclick={() => (open = false)}>Cancel</button>
      <button
        onclick={() => {
          onSubmit(link);
          open = false;
          link = "";
        }}
        disabled={!link}
      >
        Add
      </button>
    </div>
  </div>
{/if}
