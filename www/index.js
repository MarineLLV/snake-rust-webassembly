import init, { greet } from "snake_game";

init().then(_ => {
    greet("Marine");
    console.log("Everything ok!");
})