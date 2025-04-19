<script lang="ts">
    import hotkeys from "hotkeys-js";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import CreateLecture from "./CreateLecture.svelte";
    export let item: string;
    let newLecture = "";
    let searchTerm = "";
    let showPopup = false;
    let choosen = 0;
    let isSidebarOpen = true;
    let sidebarWidth = 256; // Default width in pixels (same as w-64)
    let isResizing = false;
    let minWidth = 180;
    let maxWidth = 400;

    interface Item {
        name: string;
        id: string;
        type: string; // "Lecture" or "Book" or other type
        color?: string; // For the colored dot
        updated: string;
    }

    let items: Item[] = [
        {
            name: "Operating Systems",
            id: "os1",
            type: "Lecture",
            color: "#22c55e",
            updated: "",
        },
        {
            name: "Compilers T&T",
            id: "comp1",
            type: "Book",
            color: "#4f46e5",
            updated: "",
        },
    ];

    onMount(async () => {
        try {
            const lectures = await invoke("get_lectures");
            // Transform lectures into items if needed
            // items = [...lectures.map(l => ({...l, type: "Lecture"}))];
        } catch (e) {
            console.error("Failed to load lectures:", e);
        }

        // Add mouse event listeners for resizing
        document.addEventListener("mousemove", handleMouseMove);
        document.addEventListener("mouseup", handleMouseUp);

        return () => {
            // Clean up event listeners on component unmount
            document.removeEventListener("mousemove", handleMouseMove);
            document.removeEventListener("mouseup", handleMouseUp);
        };
    });

    function startResize(e: MouseEvent) {
        e.preventDefault();
        isResizing = true;
    }

    function handleMouseMove(e: MouseEvent) {
        if (!isResizing) return;

        let newWidth = e.clientX;

        // Enforce min and max constraints
        if (newWidth < minWidth) newWidth = minWidth;
        if (newWidth > maxWidth) newWidth = maxWidth;

        sidebarWidth = newWidth;
    }

    function handleMouseUp() {
        isResizing = false;
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.code === "KeyB" && e.ctrlKey) {
            e.preventDefault();
            isSidebarOpen = !isSidebarOpen;
        } else if (e.code === "Escape" && showPopup) {
            showPopup = false;
        }
    }

    function handleAddClick() {
        showPopup = true;
    }

    $: filteredItems = items
        ? items.filter((item) =>
              item.name.toLowerCase().includes(searchTerm.toLowerCase()),
          )
        : [];
</script>

<svelte:window on:keydown={onKeyDown} />

<!-- Resizable sidebar with dynamic width -->
<div class="flex h-screen">
    <div
        class="{isSidebarOpen
            ? 'flex'
            : 'hidden'} h-full bg-gray-900 text-gray-300 shadow-lg flex-col relative"
        style="width: {sidebarWidth}px;"
    >
        <div class="p-4 flex flex-col h-full">
            <!-- Search bar with add button like in the image -->
            <div class="flex items-center mb-6 space-x-2">
                <div class="relative flex-grow">
                    <div
                        class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none text-gray-500"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            />
                        </svg>
                    </div>
                    <input
                        type="text"
                        placeholder="Search"
                        bind:value={searchTerm}
                        class="w-full py-2 pl-10 pr-4 rounded-full bg-gray-800 border-0 text-gray-300 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    />
                </div>
                <button
                    on:click={handleAddClick}
                    class="flex items-center justify-center h-10 w-10 rounded-full bg-indigo-500 hover:bg-indigo-600 transition-colors text-white"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-5 w-5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 4v16m8-8H4"
                        />
                    </svg>
                </button>
            </div>

            <!-- "All Pages" header as shown in the image -->
            <div class="mb-4 px-2 text-gray-400 font-medium">All Pages</div>

            <!-- Item List with colored dots and subtitles -->
            <div class="flex-grow overflow-y-auto">
                {#if items && items.length > 0}
                    <div class="space-y-1">
                        {#each filteredItems as itemData}
                            <button
                                class="w-full text-left p-3 rounded-lg hover:bg-gray-800 transition-colors duration-200 flex flex-col"
                                class:bg-gray-800={item === itemData.id}
                                on:click={() => {
                                    item = itemData.id;
                                }}
                            >
                                <div class="flex items-center">
                                    <div
                                        class="w-3 h-3 rounded-full mr-3"
                                        style="background-color: {itemData.color};"
                                    ></div>
                                    <span class="font-medium text-gray-200"
                                        >{itemData.name}</span
                                    >
                                </div>
                                <div class="pl-6 text-sm text-gray-500">
                                    {itemData.type}
                                </div>
                            </button>
                        {/each}
                    </div>
                {:else}
                    <div class="text-center py-4 italic text-gray-500">
                        No items found
                    </div>
                {/if}
            </div>
        </div>

        <!-- Resize handle -->
        <div
            class="absolute top-0 right-0 w-1 h-full cursor-ew-resize hover:bg-indigo-500 opacity-0 hover:opacity-100 transition-opacity"
            on:mousedown={startResize}
            role="separator"
            aria-orientation="vertical"
            aria-label="Resize sidebar"
        ></div>
    </div>

    <!-- This can be where your main content goes -->
    <div class="flex-grow">
        <!-- Main content here -->
    </div>
</div>

{#if showPopup}
    <CreateLecture {showPopup} {newLecture} {items} />
{/if}
