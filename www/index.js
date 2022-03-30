import init, { World } from "snake_game";

init().then(_ => {
    const CELL_SIZE = 10;

    const world = World.new();
    const worldWidth = world.width();

    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    function drawWorld() {
        ctx.beginPath();

        // columns
        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0);
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
        }

        // rows
        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y);
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
        }

        ctx.stroke();
    }

    function drawSnake() {
        const snakeIndex = world.snake_head_index();
        const col = snakeIndex % worldWidth;
        const row = Math.floor(snakeIndex / worldWidth);

        ctx.beginPath();
        ctx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        );
        ctx.stroke();
    }

    drawWorld();
    drawSnake();

    // update world
    setInterval(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height)
        drawWorld();
        drawSnake();
        // position
        world.update();
    }, 100)
})
