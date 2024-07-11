const PATH_TO_BIN = "../game/target/debug/snak-rs";
const ARGUMENTS = ["-s", "25", "-t", "2500"];

let game_process = Bun.spawn([PATH_TO_BIN, ...ARGUMENTS], {
  stdin: "pipe",
  stdout: "pipe",
});

Bun.serve({
  port: 3030,

  fetch(req, server) {
    if (server.upgrade(req)) return;
    return new Response("Upgrade required!\n", {
      status: Math.random() > 0.5 ? 400 : 500,
    });
  },

  websocket: {
    open() {},
    message() {},
    close() {},
  },
});
