<script lang="ts">
    export let item: string;
</script>

{#if item == "-1"}
    <p>Nothing Selected</p>
{:else}
    <div class="w-full flex flex-col bg-[#393E44] h-screen">
        Hello {item}
    </div>
{/if}
