<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    export let showPopup: boolean;
    export let newLecture: string;
    export let items: Array<{
        name: string;
        id: string;
        type: string;
        color?: string;
        updated: string;
    }>;

    const dispatch = createEventDispatcher();
    let lectureType = "Lecture"; // Default type
    let lectureColor = "#22c55e"; // Default color - green
    let errorMessage = "";
    let isSubmitting = false;

    // Color options for new items
    const colorOptions = [
        { value: "#22c55e", label: "Green" },
        { value: "#4f46e5", label: "Blue" },
        { value: "#ef4444", label: "Red" },
        { value: "#f59e0b", label: "Yellow" },
        { value: "#8b5cf6", label: "Purple" },
    ];

    // Type options for new items
    const typeOptions = [
        { value: "Lecture", label: "Lecture" },
        { value: "Book", label: "Book" },
        { value: "Note", label: "Note" },
        { value: "Article", label: "Article" },
    ];

    async function handleCreateLecture() {
        if (!newLecture.trim()) {
            errorMessage = "Please enter a name for the item";
            return;
        }

        // Check if a lecture with this name already exists
        const exists = items.some(
            (item) => item.name.toLowerCase() === newLecture.toLowerCase(),
        );

        if (exists) {
            errorMessage = "An item with this name already exists";
            return;
        }

        isSubmitting = true;

        try {
            // Create a unique ID based on name and timestamp
            const newId = `${newLecture.toLowerCase().replace(/\s+/g, "-")}-${Date.now()}`;

            // Create the new item
            const newItem = {
                name: newLecture,
                id: newId,
                type: lectureType,
                color: lectureColor,
                updated: new Date().toISOString(),
            };

            // Add the new item to the local array
            const updatedItems = [...items, newItem];

            // Send the updated items array to Tauri
            await invoke("save_items", { item: newItem });

            // Update the reference to the items array
            items = updatedItems;

            // Dispatch event to parent component
            dispatch("itemCreated", newItem);

            // Close popup and reset fields
            showPopup = false;
            newLecture = "";
            errorMessage = "";
        } catch (error) {
            console.error("Failed to save item:", error);
            errorMessage = "Failed to save the item. Please try again.";
        } finally {
            isSubmitting = false;
        }
    }

    function handleCancel() {
        showPopup = false;
        newLecture = "";
        errorMessage = "";
    }
</script>

<div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    on:click={handleCancel}
>
    <div
        class="bg-gray-800 rounded-lg p-6 w-full max-w-md relative"
        on:click|stopPropagation={() => {}}
    >
        <h2 class="text-xl font-bold mb-4 text-white">Create New Item</h2>

        <div class="mb-4">
            <label
                for="lecture-name"
                class="block text-sm font-medium text-gray-300 mb-1">Name</label
            >
            <input
                id="lecture-name"
                type="text"
                bind:value={newLecture}
                placeholder="Enter name"
                class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:ring-2 focus:ring-indigo-500"
            />
            {#if errorMessage}
                <p class="text-red-400 text-sm mt-1">{errorMessage}</p>
            {/if}
        </div>

        <div class="mb-4">
            <label
                for="lecture-type"
                class="block text-sm font-medium text-gray-300 mb-1">Type</label
            >
            <select
                id="lecture-type"
                bind:value={lectureType}
                class="w-full p-2 rounded bg-gray-700 border border-gray-600 text-white focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
                {#each typeOptions as option}
                    <option value={option.value}>{option.label}</option>
                {/each}
            </select>
        </div>

        <div class="mb-6">
            <label class="block text-sm font-medium text-gray-300 mb-1"
                >Color</label
            >
            <div class="flex flex-wrap gap-2">
                {#each colorOptions as option}
                    <button
                        type="button"
                        on:click={() => (lectureColor = option.value)}
                        class="w-8 h-8 rounded-full border-2 transition-all"
                        class:border-white={lectureColor === option.value}
                        class:border-transparent={lectureColor !== option.value}
                        style="background-color: {option.value};"
                        aria-label={option.label}
                    ></button>
                {/each}
            </div>
        </div>

        <div class="flex justify-end space-x-3">
            <button
                on:click={handleCancel}
                class="px-4 py-2 rounded bg-gray-700 text-gray-300 hover:bg-gray-600 transition-colors"
                disabled={isSubmitting}
            >
                Cancel
            </button>
            <button
                on:click={handleCreateLecture}
                class="px-4 py-2 rounded bg-indigo-500 text-white hover:bg-indigo-600 transition-colors flex items-center justify-center"
                disabled={isSubmitting}
            >
                {#if isSubmitting}
                    <svg
                        class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                    >
                        <circle
                            class="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            stroke-width="4"
                        ></circle>
                        <path
                            class="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        ></path>
                    </svg>
                    Saving...
                {:else}
                    Create
                {/if}
            </button>
        </div>
    </div>
</div>
