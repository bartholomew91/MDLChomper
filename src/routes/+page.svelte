<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Renderer, Scene } from "../lib";
  import { RENDER_MODE } from "../lib/components/types/Renderer.types";
  import { cursorState } from "$lib/state.svelte";
  import { hex2float32, is_valid_hex } from "$lib/utility/hex";
  import { Menu } from "@tauri-apps/api/menu";
  import { open } from "@tauri-apps/plugin-dialog";

  const menu = Menu.new({
    items: [
      {
        id: 'open-file',
        text: 'Open File',
        action: async () => {
          const file = await open({
            multiple: false,
            directory: false,
          });
          if (file) {
            load_file(file);
          }
        }
      }
    ]
  });
  

  let hex_data = $state([]);
  let col_size = $state(16);
  let cursor_pos = $state(-1);
  let f32_value = $state(0.0);
  let highlighted_verts = $state([]);
  let hex_offset = $state('000000');

  const float_flags = [
    "40", "41", "C1", "3F",
    "C0", "BE", "3E",
  ];

  async function load_file(path: string) {
    hex_data = await invoke("load_hex", { path });
  }

  function range(size: number, startAt: number = 0) {
    return [...Array(size).keys()].map(i => i + startAt);
  }

  function change_col_size(amount: number) {
    col_size += amount;
  }

  function set_cursor_pos(index: number): void {
    cursor_pos = index;
  }

  async function byte_selection_to_f32(): number {
    const bytes_le = hex_data.slice(parseInt(cursorState.start, 10), (parseInt(cursorState.end, 10) + 1)).reverse();
    const hex = bytes_le.join('');

    if (is_valid_hex(hex)) {
      return await hex2float32(hex);
    }

    return NaN;
  }

  // highlight possible vertice data
  function analyze_for_verts() {
    hex_data.map((val, index) => {
      if(float_flags.includes(val)) {
        highlighted_verts.push(range(4, index - 3));
      }
    });
    highlighted_verts = highlighted_verts.map((val) => val).flat();
  }

  async function get_byte_selection() {
    f32_value = await byte_selection_to_f32();
  }

  $effect(async () => {
    f32_value = await byte_selection_to_f32();
  });
</script>

<main class="container">
  <div id="settings" class="panel" style="width: 24vw; margin-right: 5px;">
    <label for="col-size" style="margin-left: 5px;">Hex Editor Columns</label>
    <input type="number" bind:value={col_size} max="32" style="width: 40px;" />
    <fieldset style="display: grid; gap: 10px; grid-template-columns: repeat(2, 1fr);">
      <legend>Vertices</legend>
      <div style="grid-column: span 2;">
        <button disabled={hex_data.length === 0} onclick={analyze_for_verts}>Analyze Hex for possible vertices</button>
      </div>
      <div>
        <label for="voffset">Offset: </label>
        <input id="voffset" bind:value={hex_offset} style="width: 25%;" />
      </div>
      <div>
        <label for="vtype">Type: </label>
        <select id="vtype">
          <option value="Float">Float</option>
          <option value="Half-Float">Half-Float</option>
          <option value="Short-Signed">Short-Signed</option>
        </select>
      </div>
      <div>
        <label for="vcount">Count: </label>
        <input id="vcount" value={0} type="number" style="width: 25%;" />
      </div>
      <div>
        <label for="vformat">Type: </label>
        <select id="vformat">
          <option value="XYZ">XYZ</option>
          <option value="XZY">XZY</option>
          <option value="YXZ">YXZ</option>
          <option value="YZX">YZX</option>
          <option value="ZXY">ZXY</option>
          <option value="ZYX">ZYX</option>
        </select>
      </div>
      <div>
        <label for="vpadding">Padding: </label>
        <input id="vpadding" value={0} type="number" style="width: 25%;" />
      </div>
      <div>
        <label for="vpadinter">Interval: </label>
        <input id="vpadinter" value={0} type="number" style="width: 25%;" />
      </div>
    </fieldset>
  </div>
  <div id="hex-editor" class="panel">
    <div id="hex-editor-header">
      <!-- <div class="address">
        <span>Address</span>
      </div> -->
      <div class="hex">
        <span>00</span>
        <span>01</span>
        <span>02</span>
        <span>03</span>
        <span>04</span>
        <span>05</span>
        <span>06</span>
        <span>07</span>
        <span>08</span>
        <span>09</span>
        <span>0A</span>
        <span>0B</span>
        <span>0C</span>
        <span>0D</span>
        <span>0E</span>
        <span>0F</span>
      </div>
      <div class="ascii">
        <span>ASCII</span>
      </div>
    </div>
    <div id="hex-editor-content">
      <Renderer hex_data={hex_data} highlighted_verts={highlighted_verts} --col-size={col_size} />
      <Renderer hex_data={hex_data} mode={RENDER_MODE.Ascii} drag_enabled={false} --gap-size={'0px'} --col-size={col_size} />
    </div>
  </div>
  <div id="model-viewer" class="panel no-scroll">
    <Scene />
  </div>
</main>

<style>
  button:disabled {
    color: #818181;
    cursor: not-allowed;
    outline: transparent;
  }

#hex-editor-header {
  display: flex;
  flex-direction: row;
  gap: 10px;
}

#hex-editor-header .hex {
  display: grid;
  grid-template-columns: repeat(var(--col-size, 16), minmax(20px, auto));
  grid-column-gap: var(--gap-size, 10px);
}

#hex-editor-content {
  display: flex;
  flex-direction: row;
  gap: 10px;
}

.panel {
  max-height: 100vh;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #414141 #161616;
}

.no-scroll {
  overflow: hidden;
}

select {
  height: 40px;
  border-radius: 8px;
  background-color: #0f0f0f98;
  color: #fff;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  border: 1px solid transparent;
}

:root {
  font-family: "Source Code Pro", monospace;
  font-optical-sizing: auto;
  font-style: normal;
  font-size: 16px;
  font-weight: 400;
  margin: 0;
  padding: 0;

  color: #f6f6f6;
  background-color: #0e0e0e;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

::selection {
  background-color: transparent;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: row;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #ffffff;
  background-color: #0f0f0f98;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #0f0f0f69;
}

input,
button {
  outline: none;
}
</style>
