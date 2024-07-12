import { GameStateMessage, InitMessage } from "./types";

const BASE_URL = "ws://127.0.0.1:3030";

const APPLE_COLOR = "red";
const colorsPool = ["green", "blue", "orange", "purple"];
const playersToColors = new Map<string, string>();

let state: GameStateMessage = { state: { players: {}, ap: "0x0" } };

(function init() {
  const ws = new WebSocket(BASE_URL);

  const handleKeyPress = function (e: KeyboardEvent) {
    switch (e.key) {
      case "ArrowUp":
      case "k":
        return ws.send("up");
      case "ArrowDown":
      case "j":
        return ws.send("down");
      case "ArrowLeft":
      case "h":
        return ws.send("left");
      case "ArrowRight":
      case "l":
        return ws.send("right");
      case "Space":
        return ws.send("start");
    }
  };

  ws.onopen = function () {
    console.log("Ws connection opened");
    document.addEventListener("keydown", handleKeyPress);
  };

  ws.onmessage = function (event: MessageEvent) {
    console.log("Message received:", event.data);
    const data = event.data.toString();
    if (data.startsWith('{"init')) {
      handleInitMessage(data);
    } else if (data.startsWith('{"state')) {
      handleGameStateMessage(data);
    }
  };

  ws.onclose = function () {
    console.log("Ws connection closed");
    document.removeEventListener("keydown", handleKeyPress);
  };

  ws.onerror = function (error: Event) {
    console.error("Ws error:", error);
  };
})();

function handleInitMessage(data: string) {
  if (!document.querySelector(".grid")) {
    const parsed = JSON.parse(data) as InitMessage;
    const [width, height] = parsed.init.field.split("x").map(parseInt);
    document.querySelector("#app")!.appendChild(createGrid(width, height));
  }
}

function handleGameStateMessage(data: string) {
  const grid = document.querySelector(".grid");
  if (!grid) return;
  const newState = JSON.parse(data) as GameStateMessage;
  const changes = diff(state, newState);
  applyChangesToGrid(changes);
  state = newState;
}

function diff(
  oldState: GameStateMessage,
  newState: GameStateMessage,
): Map<string, string> {
  const changes = new Map<string, string>();

  // unpaint all deleted snakes
  for (const id in oldState.state.players) {
    if (!(id in newState.state.players)) {
      for (const cell of oldState.state.players[id].split("+")[0].split(",")) {
        changes.set(cell, "unset");
      }
      // free the color
      colorsPool.push(playersToColors.get(id)!);
      playersToColors.delete(id);
    }
  }

  // unpaint all prev tails
  for (const id in newState.state.players) {
    const prevTail = newState.state.players[id].split("+")[1];
    if (prevTail) changes.set(prevTail, "unset");
  }

  for (const id in newState.state.players) {
    if (id in oldState.state.players) {
      // calculate what to repaint
      const color = playersToColors.get(id);
      const oldBody = oldState.state.players[id].split("+")[0].split(",");
      const newBody = newState.state.players[id].split("+")[0].split(",");

      for (const cell of newBody) {
        if (color && !oldBody.includes(cell)) {
          changes.set(cell, color);
        }
      }

      for (const cell of oldBody) {
        if (!newBody.includes(cell)) {
          changes.set(cell, "unset");
        }
      }
    } else {
      // paint entire body
      const cells = newState.state.players[id].split("+")[0].split(",");
      const color = colorsPool.shift();
      if (color) {
        playersToColors.set(id, color);
        cells.forEach((cell) => changes.set(cell, color));
      }
    }
  }

  // repaint apple
  if (newState.state.ap !== oldState.state.ap) {
    changes.set(oldState.state.ap, "unset");
    changes.set(newState.state.ap, APPLE_COLOR);
  }

  return changes;
}

function createGrid(width: number, height: number) {
  const grid = document.createElement("div");
  grid.classList.add("grid");
  grid.style.aspectRatio = `${width}/${height}`;
  grid.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
  grid.style.gridTemplateRows = `repeat(${height}, 1fr)`;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const cell = document.createElement("div");
      cell.classList.add("cell");
      cell.dataset.x = x.toString();
      cell.dataset.y = y.toString();
      grid.appendChild(cell);
    }
  }
  return grid;
}

function applyChangesToGrid(changes: Map<string, string>) {
  const grid = document.querySelector(".grid");
  if (!grid) return;

  for (const [cell, color] of changes.entries()) {
    const [x, y] = cell.split("x");
    const el = grid.querySelector<HTMLDivElement>(
      `[data-x="${x}"][data-y="${y}"]`,
    );
    if (el) el.style.backgroundColor = color;
  }
}
