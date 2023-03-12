import init, { JsInput, JsState, Tetris } from "./wasm/wasm.js";

const block_size = 20;
const fps = 30;

const rendering = (tetris: Tetris, ctx: CanvasRenderingContext2D) => {
  ctx.clearRect(
    0,
    0,
    tetris.size_x() * block_size,
    tetris.size_y() * block_size,
  );
  for (let y = 0; y < tetris.size_y(); y++) {
    for (let x = 0; x < tetris.size_x(); x++) {
      const state = tetris.state(x, y);
      switch (state) {
        case JsState.Block:
          ctx.fillRect(x * block_size, y * block_size, block_size, block_size);
          break;
        default:
          ctx.fillRect(
            (x * block_size) + (block_size / 2),
            (y * block_size) + (block_size / 2),
            1,
            1,
          );
      }
    }
  }
  console.log(tetris.to_string());
};

const createPointElement = (point: number) =>
  document.createTextNode(`point: ${point}`);

export default () =>
  init().then(() => {
    const tetris = Tetris.new();

    const canvas = document.createElement("canvas");
    canvas.width = tetris.size_x() * block_size;
    canvas.height = tetris.size_y() * block_size;
    canvas.style.setProperty("border", "solid");
    document.body.appendChild(canvas);

    const info = document.createElement("div");
    info.appendChild(
      document.createTextNode("←: h / ↓: j or k / →: l / rotate: p, n, [ or ]"),
    );
    info.appendChild(document.createElement("br"));
    document.body.appendChild(info);

    let count = 0;
    info.appendChild(createPointElement(count));

    setInterval(
      () =>
        rendering(tetris, canvas.getContext("2d") as CanvasRenderingContext2D),
      1_000 / fps,
    );

    setInterval(() => {
      count += tetris.deside();
      info.removeChild(info.lastChild);
      info.appendChild(createPointElement(count));
    }, 1_000);

    self.window.addEventListener("keydown", (e: KeyboardEvent) => {
      console.log(e.key);
      switch (e.key) {
        case "ArrowLeft":
        case "h":
          tetris.input(JsInput.MoveLeft);
          break;
        case "ArrowDown":
        case "j":
          tetris.input(JsInput.MoveBottom);
          break;
        case "ArrowUp":
        case "k":
          tetris.input(JsInput.MoveTop);
          break;
        case "ArrowRight":
        case "l":
          tetris.input(JsInput.MoveRight);
          break;
        case "[":
        case "p":
          tetris.input(JsInput.RotateLeft);
          break;
        case "]":
        case "n":
          tetris.input(JsInput.RotateRight);
          break;
      }
    });
  });
