import "./style.css";

var socket;

function init() {
  socket = new WebSocket("ws://localhost:3030");

  socket.onmessage = (e) => {
    var data = e.data.toString();
    if (data.startsWith('{"init')) {
      parseInitMessage(data);
    } else if (data.startsWith('{"state')) {
      parseGameState(data);
    }
  };

  socket.onerror = (e) => {
    console.error("Error in Websocket!");
    console.error(e);
  };

  socket.onclose = () => {
    console.error("Socket disconnected!");
  };
}

init();

function parseInitMessage(data: string) {}
function parseGameState(data: string) {}

function createGrid(width: number, height: number) {}
