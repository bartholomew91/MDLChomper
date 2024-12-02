<script lang="ts">
    import { cursorState } from "../state.svelte";
    import { hex2num, is_hex_ascii, is_hex_zeroed, parse_ascii } from "../utility/hex";
    import { RENDER_MODE } from "./types/Renderer.types";

    interface RendererProps {
        hex_data: Array<string>,
        mode?: RENDER_MODE,
        drag_enabled?: boolean,
        highlighted_verts?: Array<number>
    };

    /**
     * Renderer for the Hex/Ascii explorer panel
     * @prop {Array<string>} hex_data
     * @prop {RENDER_MODE} mode
     * @prop {boolean} drag_enabled
    */
    let { 
        hex_data,
        mode = RENDER_MODE.Hex,
        drag_enabled = true,
        highlighted_verts = []
    }: RendererProps = $props();

    let is_selecting = $state(false);

    /**
     * Sets up functions for how we handle rendering
     * based on the set RENDER_MODE. These can be either
     * Hex or Ascii.
     * 
     * When in Hex mode, 0x00 bytes are greyed out in the
     * renderer and the value is displayed in hex form
     * without the 0x prefix.
     * 
     * When in Ascii mode, 0x00 AND non-ascii characters are
     * greyed out in the renderer. The value is displayed in
     * ascii, if it's valid. If it's not valid a • is displayed
     * instead.
     */
    const renderModeMap = {
        [RENDER_MODE.Hex]: (hex: string) => ({
            value: hex,
            isDark: is_hex_zeroed(hex)
        }),
        [RENDER_MODE.Ascii]: (hex: string) => ({
            value: parse_ascii(hex2num(hex)),
            isDark: is_hex_ascii(hex)
        })
    };

    /**
     * Handles selecting byte data with the mouse cursor in the renderer
     * and updates the global cursorState for the start/end index of the selection
     */
    function handle_drag_select(): void {
        if (!is_selecting) return;

        const window_selection = window.getSelection();

        if (!window_selection) return;

        const { anchorNode, focusNode } = window_selection;

        if (!anchorNode || !focusNode) return;

        const start = anchorNode?.parentNode?.dataset?.id;
        const end = focusNode?.parentNode?.dataset?.id

        if (start && end) {
            cursorState.start = start;
            cursorState.end   = end;
        }
    }

    /**
     * If index is within the start/end range of the
     * curorState return true
     * @param index
     */
    function should_highlight(index: number): boolean {
        const { start, end } = cursorState;
        return start <= index && end >= index;
    }

    function select_byte(index: number) {
        cursorState.start = index;
        cursorState.end = index;
    }

    if (hex_data.length === 0) {
        hex_data = new Array(1024).fill("00");
    }

    let hex_renderer: HTMLDivElement;
    $effect(() => {
        /**
         * setup our event listeners to handle selecting hex data
         * with the mouse cursor
         * */
        if (hex_renderer && drag_enabled) {
            const handleMouseDown = () => is_selecting = true;
            const handleMouseMove = handle_drag_select;
            const handleMouseUp   = () => is_selecting = false;

            hex_renderer.addEventListener('mousedown', handleMouseDown);
            hex_renderer.addEventListener('mousemove', handleMouseMove);
            hex_renderer.addEventListener('mouseup', handleMouseUp);

            return () => {
                hex_renderer.removeEventListener('mousedown', handleMouseDown);
                hex_renderer.removeEventListener('mousemove', handleMouseMove);
                hex_renderer.removeEventListener('mouseup', handleMouseUp);
            };
        }
    });
</script>
<div bind:this={hex_renderer} class:small-spacing={mode === RENDER_MODE.Ascii}>
    {#if hex_data.length > 0}
        {#each hex_data as hex, i}
            {@const { value, isDark } = renderModeMap[mode](hex)}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <span
                class="byte"
                data-id={i}
                onclick={select_byte(i)}
                class:vert-highlight={highlighted_verts.includes(i)}
                class:highlighted={should_highlight(i)}
                class:darkFont={isDark}
                alt={`Offset: ${parseInt(i, 16)}`}
            >
                {value}
            </span>
        {/each}
    {/if}
</div>

<style>
div {
    display: grid;
    grid-template-columns: repeat(var(--col-size, 16), minmax(20px, auto));
    grid-column-gap: var(--gap-size, 10px);
}
.small-spacing {
    grid-column-gap: -5px;
}
.small-spacing span {
    display: block;
    width: 10px;
}
.darkFont {
  color: #818181;
}
.highlighted {
  background-color: #24c8db;
}
.vert-highlight {
    background-color: deeppink;
}
::selection {
  background-color: transparent;
}
</style>