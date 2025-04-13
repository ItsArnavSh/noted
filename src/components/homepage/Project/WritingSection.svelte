<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import markdownit from "markdown-it";
    const md = markdownit();
    let currentChunk = 0; // First chunk is being edited by default
    let sectionPos: Number[] = [];
    interface Chunk {
        text: string;
    }

    // Start with one chunk that's being edited
    let chunks: Chunk[] = [{ text: "# Hello Markdown!" }];

    function editChunk(index: number) {
        currentChunk = index;
        // Force update
        chunks = [...chunks];
    }

    function finishEditing() {
        // Do nothing, always keep a chunk in edit mode
    }

    function handleTextareaKeydown(e: KeyboardEvent, index: number) {
        const textarea = e.target as HTMLTextAreaElement;
        const cursorPosition = textarea.selectionStart;
        const selectionEnd = textarea.selectionEnd;
        const text = textarea.value;

        // Handle navigation and editing keys
        switch (e.code) {
            case "ArrowUp":
                // If at the first line, try to move to the previous chunk
                if (
                    cursorPosition <= text.indexOf("\n") ||
                    text.indexOf("\n") === -1
                ) {
                    if (index > 0) {
                        e.preventDefault();
                        navigationDirection = "end";
                        editChunk(index - 1);
                    }
                }
                break;

            case "ArrowDown":
                // If at the last line, try to move to the next chunk
                const lastNewlinePos = text.lastIndexOf("\n");
                if (lastNewlinePos === -1 || cursorPosition > lastNewlinePos) {
                    if (index < chunks.length - 1) {
                        e.preventDefault();
                        navigationDirection = "start";
                        editChunk(index + 1);
                    }
                }
                break;

            case "ArrowLeft":
                // If at the beginning of text, try to move to end of previous chunk
                if (cursorPosition === 0) {
                    if (index > 0) {
                        e.preventDefault();
                        navigationDirection = "end";
                        editChunk(index - 1);
                    }
                }
                break;

            case "ArrowRight":
                // If at the end of text, try to move to beginning of next chunk
                if (cursorPosition === text.length) {
                    if (index < chunks.length - 1) {
                        e.preventDefault();
                        navigationDirection = "start";
                        editChunk(index + 1);
                    }
                }
                break;

            case "Enter":
                if (e.ctrlKey) {
                    // Add a new empty chunk at the end instead of finishing editing
                    e.preventDefault();
                    chunks = [...chunks, { text: "" }];
                    navigationDirection = "start";
                    editChunk(chunks.length - 1);
                } else {
                    // Split the current chunk at cursor position
                    e.preventDefault();

                    const textBeforeCursor = text.substring(0, cursorPosition);
                    const textAfterCursor = text.substring(cursorPosition);

                    // Update current chunk with text before cursor
                    chunks[index].text = textBeforeCursor;

                    // Create new chunk with text after cursor
                    const newChunks = [...chunks];
                    newChunks.splice(index + 1, 0, { text: textAfterCursor });
                    chunks = newChunks;

                    // Focus the new chunk
                    navigationDirection = "start";
                    editChunk(index + 1);
                }
                break;

            case "Backspace":
                // If at the beginning of the chunk and the chunk is empty
                if (cursorPosition === 0 && selectionEnd === 0) {
                    if (text.trim() === "" && chunks.length > 1) {
                        // Always keep at least one chunk
                        // Delete the empty chunk
                        e.preventDefault();
                        const newChunks = [...chunks];
                        newChunks.splice(index, 1);
                        chunks = newChunks;

                        // Move to previous chunk if it exists
                        if (index > 0) {
                            navigationDirection = "end";
                            editChunk(index - 1);
                        } else if (newChunks.length > 0) {
                            navigationDirection = "start";
                            editChunk(0);
                        }
                    } else if (index > 0) {
                        // Merge with previous chunk
                        e.preventDefault();

                        const previousChunkText = chunks[index - 1].text;
                        const mergedText = previousChunkText + text;

                        // Update previous chunk
                        chunks[index - 1].text = mergedText;

                        // Remove current chunk
                        const newChunks = [...chunks];
                        newChunks.splice(index, 1);
                        chunks = newChunks;

                        // Focus the previous chunk with cursor at merge point
                        navigationDirection = "merge";
                        mergePosition = previousChunkText.length;
                        editChunk(index - 1);
                    }
                }
                break;
        }
    }

    // For cursor positioning
    let navigationDirection = "end";
    let mergePosition = 0;

    function positionCursor(node) {
        if (node) {
            node.focus();

            // Set cursor position based on navigation direction
            if (navigationDirection === "end") {
                node.selectionStart = node.value.length;
                node.selectionEnd = node.value.length;
            } else if (navigationDirection === "start") {
                node.selectionStart = 0;
                node.selectionEnd = 0;
            } else if (navigationDirection === "merge") {
                node.selectionStart = mergePosition;
                node.selectionEnd = mergePosition;
            }

            // Reset to default for next time
            navigationDirection = "end";
        }
    }
</script>

<div class="flex flex-col text-white m-10">
    {#each chunks as chunk, i}
        {#if currentChunk === i}
            <textarea
                class="bg-transparent border-none outline-none focus:outline-none focus:ring-0 focus:border-transparent font-mono resize-none"
                bind:value={chunks[i].text}
                use:positionCursor
                on:keydown={(e) => handleTextareaKeydown(e, i)}
            />
        {:else}
            <div
                class="markdown rounded cursor-pointer"
                on:click={() => {
                    navigationDirection = "end";
                    editChunk(i);
                }}
            >
                {@html md.render(chunk.text)}
            </div>
        {/if}
    {/each}
</div>
