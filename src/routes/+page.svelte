<script lang="ts">
    import Project from "../components/homepage/Project.svelte";
    import SideBar from "../components/homepage/Sidebar/SideBar.svelte";
    let item = "0";
</script>

<div class="flex flex-row w-full">
    <SideBar bind:item />
    <Project bind:item />
</div>
