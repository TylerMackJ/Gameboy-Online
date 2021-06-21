import { Gameboy } from "gameboy";
import { memory } from "gameboy/gameboy_bg";

const gameboy = Gameboy.new();

const WIDTH = 256;
const HEIGHT = 256;
const PIXEL_SIZE = 1;

const canvas = document.getElementById("gameboy-canvas");
const ctx = canvas.getContext("2d");
ctx.canvas.width = WIDTH * PIXEL_SIZE;
ctx.canvas.height = HEIGHT * PIXEL_SIZE;

const COLOR0 = "#000000"
const COLOR1 = "#555555"
const COLOR2 = "#AAAAAA"
const COLOR3 = "#FFFFFF"

let started = false;

const renderLoop = () => {
    if (started) {
        //console.log(gameboy.registers.get_pc());
        gameboy.step();
        drawScreen();
    }
    requestAnimationFrame(renderLoop);
}

const getIndex = (row, col) => {
    return row * WIDTH + col;
}

const drawScreen = () => {
    const pixelsPtr = gameboy.frame();
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, WIDTH * HEIGHT);

    ctx.beginPath();

    for (let row = 0; row < HEIGHT; row++) {
        for (let col = 0; col < WIDTH; col++) {
            let idx = getIndex(row, col);

            switch (pixels[idx]) {
                case 0x00: ctx.fillStyle = COLOR0;
                case 0x01: ctx.fillStyle = COLOR1;
                case 0x10: ctx.fillStyle = COLOR2;
                case 0x11: ctx.fillStyle = COLOR3;
            }

            ctx.fillRect(
                col * PIXEL_SIZE,
                row * PIXEL_SIZE,
                PIXEL_SIZE,
                PIXEL_SIZE
            );
        }
    }

    ctx.stroke();
}

const handleFile = (e) => {
    const file = e.currentTarget.files[0];
    document.getElementById("file-name").textContent = e.currentTarget.files[0].name;
    const reader = new FileReader();
    reader.onload = (e) => {
        var contents = e.target.result;
        gameboy.load_rom(contents);
        started = true;
    }
    reader.readAsBinaryString(file);
}

document.getElementById("file-input").addEventListener('change', handleFile);

requestAnimationFrame(renderLoop)