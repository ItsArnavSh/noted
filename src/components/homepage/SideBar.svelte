<script lang="ts">
    import { onMount } from "svelte";
    import Projects from "./Sidebar/Projects.svelte";
    import { invoke } from "@tauri-apps/api/core";
    export let item: string;
    let newLecture = "";
    let searchTerm = "";
    let showPopup = false;
    let choosen = 0;
    let isSidebarOpen = false;

    interface Lecture {
        name: string;
        lid: string;
        updated: string;
    }

    let lectures: Lecture[];

    onMount(async () => {
        lectures = await invoke("get_lectures");
    });

    async function addLecture() {
        if (newLecture == "") {
            alert("Cannot Create Empty Lecture");
            return;
        }
        try {
            await invoke("insert_lecture", {
                name: newLecture,
            });
            console.log("Lecture inserted!");
            newLecture = ""; // Clear input after adding
            showPopup = false; // Close popup after adding
        } catch (e) {
            console.error("Failed to insert lecture:", e);
        }
        lectures = await invoke("get_lectures");
        console.log(lectures);
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.code === "KeyB" && e.ctrlKey) {
            e.preventDefault();
            isSidebarOpen = !isSidebarOpen;
        } else if (e.code === "Escape" && showPopup) {
            showPopup = false;
        }
    }

    $: filteredLectures = lectures
        ? lectures.filter((lecture) =>
              lecture.name.toLowerCase().includes(searchTerm.toLowerCase()),
          )
        : [];
</script>

<svelte:window on:keydown={onKeyDown} />

<div
    class="{isSidebarOpen
        ? 'w-64'
        : 'hidden'} h-screen bg-[#79A89E] text-white shadow-lg flex flex-col"
>
    <div class="p-4 flex flex-col h-full">
        <!-- Header with + button -->
        <div
            class="flex justify-between items-center mb-4 border-b border-white pb-2"
        >
            <div class="text-xl font-bold">Lectures</div>
            <button
                class="text-white hover:bg-[#5A8A80] p-1 rounded-full w-6 h-6 flex items-center justify-center transition-colors"
                on:click={() => (showPopup = true)}
                title="Add new lecture"
            >
                <span class="text-lg font-bold">+</span>
            </button>
        </div>

        <!-- Search -->
        <div class="mb-4">
            <input
                type="text"
                placeholder="Search lectures..."
                bind:value={searchTerm}
                class="w-full p-2 rounded text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
        </div>

        <!-- Lecture List -->
        <div class="flex-grow overflow-y-auto">
            {#if lectures && lectures.length > 0}
                <div class="space-y-2">
                    {#each filteredLectures as lecture, i}
                        <button
                            class="w-full text-left p-2 rounded hover:bg-[#5A8A80] transition-colors duration-200 flex justify-between items-center"
                            class:bg-[#5A8A80]={item === lecture.lid}
                            on:click={() => {
                                item = lecture.lid;
                            }}
                        >
                            <span class="truncate">{lecture.name}</span>
                        </button>
                    {/each}
                </div>
            {:else}
                <div class="text-center py-4 italic">No lectures found</div>
            {/if}
        </div>
    </div>
</div>
{#if showPopup}
    <div class="fixed inset-0 z-40">
        <!-- Background overlay -->
        <div
            class="absolute inset-0 bg-black/40 backdrop-blur-sm"
            on:click={() => (showPopup = false)}
        />

        <!-- Popup modal -->
        <div class="relative z-50 flex items-center justify-center h-full">
            <div
                class="bg-white rounded-lg p-6 w-80 shadow-xl"
                on:click|stopPropagation
            >
                <h2 class="text-xl font-bold mb-4 text-gray-800">
                    Add New Lecture
                </h2>
                <div class="mb-4">
                    <input
                        bind:value={newLecture}
                        placeholder="Lecture name"
                        class="w-full p-2 border border-gray-300 rounded text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        autofocus
                    />
                </div>
                <div class="flex justify-end space-x-2">
                    <button
                        class="px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded text-gray-800 transition-colors"
                        on:click={() => (showPopup = false)}
                    >
                        Cancel
                    </button>
                    <button
                        class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
                        on:click={addLecture}
                    >
                        Add
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
