<script lang="ts">
    import { preventDefault } from "svelte/legacy";
    import { render } from "svelte/server";

    interface Section {
        title: string;
        lines: string[];
    }
    interface Coords {
        secid: number;
        lineid: number;
    }
    interface CursorInfo {
        position: number;       // The current cursor position
        before: string;         // Text before the cursor
        after: string;          // Text after the cursor
        beforeLength: number;   // Length of text before the cursor
        afterLength: number;    // Length of text after the cursor
    }
    let cursorData : CursorInfo;
    function getCursorInfo(textarea: HTMLTextAreaElement): CursorInfo {
        const cursorPos = textarea.selectionStart;
        const fullText = textarea.value;

        const beforeCursor = fullText.slice(0, cursorPos);
        const afterCursor = fullText.slice(cursorPos);

        return {
            position: cursorPos,
            before: beforeCursor,
            after: afterCursor,
            beforeLength: beforeCursor.length,
            afterLength: afterCursor.length,
        };
    }

    let currInp = "";
    let Active: Coords = { secid: 0, lineid: 1 }; //-1 lineid means the heading is selected
    //Activate a given chunk when clicked or gone to, and remove the older chunk as active
    function ActivateLine(coord: Coords){
        //Save current active data before switching
        if (Active.lineid === -1) {
            data[Active.secid].title = currInp;
        } else if (Active.lineid >= 0) {
            data[Active.secid].lines[Active.lineid] = currInp;
        }
        //Remove if blurred is empty
        if(typeof currInp!='string' ||  currInp.trim()==""){
          deleteEmpty(Active.secid,Active.lineid)
        }

        //Set new active item
        Active = coord;

        //Set current input value based on new active item
        if (Active.lineid === -1) {
            currInp = data[Active.secid].title;
        } else {
            currInp = data[Active.secid].lines[Active.lineid];
        }

        // Allow the DOM to update before resizing
        setTimeout(adjustTextareaHeight, 0);
    }

    //Auto-resize textareas to fit content exactly
    function autoResize(e: Event) {
        const textarea = e.target as HTMLTextAreaElement;
        adjustTextareaElement(textarea);
    }

    function adjustTextareaHeight() {
        const activeTextarea = document.querySelector("textarea");
        if (activeTextarea) {
            adjustTextareaElement(activeTextarea);
        }
    }

    function adjustTextareaElement(textarea: HTMLTextAreaElement) {
        // Reset height to calculate proper scrollHeight
        textarea.style.height = "0";
        // Set to exactly the content height
        textarea.style.height = textarea.scrollHeight + "px";
    }

    function keyPress(e: KeyboardEvent) {
        switch (e.key) {
            case "Enter":
                e.preventDefault();
                //Save the previous state
                saveState("add")
                if (e.shiftKey) {
                    //Shift+Enter = New Section
                    NewSection();
                    return;
                }
                NewLine();
                break;
            case "Backspace":
                if(!cursorData.beforeLength)
                  saveState("delete")
                  DeleteBlock();
                break;
            case "ArrowUp":
              GoUp();
            break;
            case "ArrowDown":
              e.preventDefault();
              GoDown();
              break;
        }
    }
    function GoUp(){
      let {secid,lineid} = Active
      if(secid==0 && lineid==-1)//Already at the top
        return
      else if(lineid!=-1){
        ActivateLine({secid:secid,lineid:lineid-1})
      }
      else{//In some section's heading
        let newsec = secid-1;
        let newlineid = data[newsec].lines.length-1;//The last line of prev section
        ActivateLine({secid:newsec,lineid:newlineid})
      }
    }
    function GoDown() {
        let { secid, lineid } = Active;

        // If currently on a line
        if (lineid !== -1) {
            // If not at the last line of the section
            if (lineid < data[secid].lines.length - 1) {
                ActivateLine({ secid, lineid: lineid + 1 });
            }
            // If at the last line, go to section title of next section (if exists)
            else if (secid < data.length - 1) {
                ActivateLine({ secid: secid + 1, lineid: -1 });
            }
        }
        // If currently on a section heading
        else {
            // Go to first line of that section if it exists
            if (data[secid].lines.length > 0) {
                ActivateLine({ secid, lineid: 0 });
                return
            }
            // Else move to the next section heading (if any)
            else if (secid < data.length - 1) {
                ActivateLine({ secid: secid + 1, lineid: -1 });
            }
        }
    }
    function deleteEmpty(secid:number,lineid:number){

      data[secid].lines.splice(lineid,1)
      data = [...data]//Refresh
    }
    function NewSection() {
        data = [...data, { title: "", lines: [] }];
        ActivateLine({ lineid: -1, secid: data.length - 1 });
        currInp = ""
    }
    function NewLine() {
        const { secid, lineid } = Active;
        const before = cursorData.before;
        const after = cursorData.after;

        if (lineid === -1) {
            // Inside section heading → insert new line at top
            data[secid].lines.unshift("");

            ActivateLine({ secid, lineid: 0 });
            currInp = "";
            return;
        }

        // You're in a line, so split it into two at the cursor
        currInp = before;

        const newLines = [
            ...data[secid].lines.slice(0, lineid),
            before,
            after,
            ...data[secid].lines.slice(lineid + 1)
        ];

        data[secid].lines = newLines;
        ActivateLine({ secid, lineid: lineid + 1 });
    }





    function DeleteBlock() {
        const { secid, lineid } = Active;
        if (secid === -1 && lineid === -1) return; // Title (or top-level) can't be deleted

        const after = cursorData.after;

        if (lineid === -1) {
            // You're on a section heading
            if (secid > 0) {
                const prevSec = secid - 1;
                const prevLines = data[prevSec].lines;
                if (prevLines.length > 0) {
                    const lastLine = prevLines.length - 1;
                    data[prevSec].lines[lastLine] += after; // Append after content if exists
                    ActivateLine({ secid: prevSec, lineid: lastLine });
                } else {
                    data[prevSec].lines = [after]; // If no lines, start one
                    ActivateLine({ secid: prevSec, lineid: 0 });
                }
                data.splice(secid, 1); // Delete current section
            }
            return;
        }

        if (lineid === 0) {
            // First line of section → move to section heading
            data[secid].lines.splice(0, 1);
            data[secid].title += after;
            ActivateLine({ secid, lineid: -1 });
            return;
        }

        // Merge into previous line
        const prevLine = data[secid].lines[lineid - 1];
        data[secid].lines[lineid - 1] = prevLine + currInp + after;
        data[secid].lines.splice(lineid, 1);
        ActivateLine({ secid, lineid: lineid - 1 });
    }


    //So the basic structure is a data divided in topics and each topic having bulletpoints
    let data: Section[] = [
        {
            title: "Introduction to Rust",
            lines: [],
        },
    ];
//Send update to backend
    function saveState(action:string){
      let {secid,lineid} = Active
      switch(action){
        case "delete":

        break;
        case "add":

        break;
      }
    }
</script>

<div
    class="ml-10 text-white space-y-8 font-sans w-full max-w-3xl h-screen overflow-y-auto"
>
    {#each data as section, secid}
        <div class="section-container">
            <div class="heading-container w-full">
                {#if secid == Active.secid && Active.lineid == -1}
                    <textarea
                        bind:value={currInp}
                        oninput={autoResize}
                        onkeydown={(e) => {
                          const textarea = e.target as HTMLTextAreaElement;
                                 const cursorInfo = getCursorInfo(textarea);
                                 cursorData = cursorInfo
                                 console.log("Change")

                        }}
                        autofocus
                        class="text-3xl font-bold bg-transparent border-none outline-none focus:outline-none focus:ring-0 resize-none w-full overflow-hidden p-2"
                    ></textarea>
                {:else}
                    <h2
                        class="text-xl font-bold p-2 hover:bg-zinc-800 rounded cursor-pointer transition-colors duration-150"
                        onclick={() => ActivateLine({ secid, lineid: -1 })}
                    >
                        {section.title}
                    </h2>
                {/if}
            </div>

            <ul
                class="list-disc list-outside pl-6 space-y-2 text-zinc-300 w-full"
            >
                {#each section.lines as line, lineid}
                    <li class="relative w-full">
                        {#if secid == Active.secid && lineid == Active.lineid}
                            <div class="w-full pr-4">
                                <textarea
                                    bind:value={currInp}
                                    oninput={autoResize}
                                    class="bg-transparent border-none outline-none focus:outline-none focus:ring-0 resize-none w-full py-1 px-2 text-lg overflow-hidden"
                                    autofocus
                                    onkeydown={(e) => {
                                            const textarea = e.target as HTMLTextAreaElement;
                                             const cursorInfo = getCursorInfo(textarea);
                                             cursorData = cursorInfo
                                        keyPress(e);
                                    }}

                                ></textarea>
                            </div>
                        {:else}
                            <div
                                class="text-lg py-1 px-2 hover:bg-zinc-800 rounded cursor-pointer transition-colors duration-150 w-full"
                                onclick={() => ActivateLine({ secid, lineid })}
                            >
                                {line}
                            </div>
                        {/if}
                    </li>
                {/each}
            </ul>
        </div>
    {/each}
    <div class="absolute bottom-0 left-0 right-0 bg-gray-800 text-white px-4 py-2 shadow-lg">
      <div class="flex justify-between items-center">
        <span>Coordinates: {Active.secid}:{Active.lineid}</span>
      </div>
    </div>

</div>

<style>
    /* Global styles */
    :global(body) {
        background-color: #121212;
    }

    /* Local component styles */
    .section-container {
        border-left: 2px solid rgba(255, 255, 255, 0.1);
        padding-left: 12px;
        width: 100%;
    }

    textarea {
        display: block;
        width: 100%;
        line-height: 1.5;
        background-color: rgba(255, 255, 255, 0.05);
        border-radius: 4px;
        margin: 0;
        padding: 0.25rem 0.5rem;
        box-sizing: border-box;
    }

    /* Ensure list items take full width */
    li {
        width: 100%;
        display: flex;
    }

    li > div {
        flex-grow: 1;
    }

    /* Make list item markers visible against dark background */
    ul {
        margin-top: 8px;
        width: 100%;
        box-sizing: border-box;
    }
</style>
