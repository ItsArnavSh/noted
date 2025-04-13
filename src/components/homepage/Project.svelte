<script lang="ts">
    import QuickWrite from "./Project/QuickWrite.svelte";
    import WritingSection from "./Project/WritingSection.svelte";
    export let item: string;
</script>

{#if item == "-1"}
    <p>Nothing Selected</p>
{:else}
    <div class="w-full flex flex-col bg-[#393E44] h-screen">
        Hello {item}
        <QuickWrite />
    </div>
{/if}
