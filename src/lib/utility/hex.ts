import { invoke } from "@tauri-apps/api/core";

function is_hex_ascii(hex: string): boolean {
    return is_hex_zeroed(hex) || 
            (hex2num(hex) <= 0 || hex2num(hex) > 127);
}

function is_hex_zeroed(hex: string): boolean {
    return hex === "00";
}

function hex2num(hex: string): number {
    if(hex.indexOf('0x') < 0) {
        return Number("0x" + hex);
    }
    return Number(hex);
}

function parse_ascii(char_code: number) {
    let string = String.fromCharCode(char_code);

    if (char_code > 0 && char_code <= 127) {
        return string;
    }
    return "·";
}

function is_valid_hex(hex: string): boolean {
    const x = parseInt(hex, 16);
    return x.toString(16) === hex.toLowerCase();
}

async function hex2float32(str: string): Promise<number> {
    let float: number = await invoke('hex_to_f32', { hex: str });
    return float;
}

export { is_hex_ascii, is_hex_zeroed, hex2num, parse_ascii, hex2float32, is_valid_hex };