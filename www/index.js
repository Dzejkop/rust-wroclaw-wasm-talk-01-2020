// wasm-bindgen handles initializing modules for us
// so from our point of view, we only have to import the functions and objects of interest

// eslint-disable-next-line import/no-unresolved
import { GameOfLife, Cell, validate_rule } from 'wasm-conway'; // eslint-disable-line camelcase
// eslint-disable-next-line import/no-unresolved

// We need direct access to module memory for quick rendering
import { memory } from 'wasm-conway/wasm_conway_bg';

const DEAD_COLOR = '#FFF';
const ALIVE_COLOR = '#000';

// `let` because these values are mutable
let cellSize = 4;
let tickSpeed = 100;

const W = 128;
const H = 128;

let gameOfLife = GameOfLife.new_with_seed(123, W, H);

const canvas = document.getElementById('main');
canvas.height = (cellSize + 1) * gameOfLife.height() + 1;
canvas.width = (cellSize + 1) * gameOfLife.width() + 1;

const ctx = canvas.getContext('2d');

const main = () => {
  const w = gameOfLife.width();
  const h = gameOfLife.height();

  // pointer to the cells array in WASM memory
  const cellsOffset = gameOfLife.cells();
  // JS Buffer to cells array
  const cellsMem = new Uint8Array(memory.buffer, cellsOffset, w * h);

  ctx.beginPath();
  for (let row = 0; row < h; row += 1) {
    for (let col = 0; col < w; col += 1) {
      const offset = row * h + col;
      const cell = cellsMem[offset];

      ctx.fillStyle = cell === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * cellSize,
        row * cellSize,
        cellSize,
        cellSize,
      );
    }
  }

  gameOfLife.tick();

  setTimeout(main, tickSpeed);
};

// The stuff below handles page inputs and such
// don't worry to much about it

const tickSpeedSlider = document.querySelector('#tickSpeed');
tickSpeedSlider.addEventListener('change', () => {
  tickSpeed = tickSpeedSlider.value;
});

tickSpeed = parseInt(tickSpeedSlider.value, 10);

const cellSizeInput = document.querySelector('#cellSize');
cellSizeInput.addEventListener('change', () => {
  cellSize = parseInt(cellSizeInput.value, 10);
  canvas.height = (cellSize + 1) * gameOfLife.height() + 1;
  canvas.width = (cellSize + 1) * gameOfLife.width() + 1;
});

const rulesWarning = document.querySelector('#rulesWarning');
rulesWarning.style.display = 'none';

const rulesInput = document.querySelector('#rules');
const rulesButton = document.querySelector('#rulesButton');
rulesButton.addEventListener('click', () => {
  const rules = rulesInput.value;

  if (!validate_rule(rules)) {
    rulesWarning.style.display = 'inline';
    return;
  }
  rulesWarning.style.display = 'none';

  gameOfLife = GameOfLife.new_with_seed_and_rules(rules, Math.random() * 1000, W, H);
});

// the only thing remaining is to start our main function
requestAnimationFrame(main);
