import init, { World } from 'snake_game'

init().then((_) => {
  const CELL_SIZE = 10
  const WORLD_WIDTH = 8
  const snake_spawn_index = Date.now() % (WORLD_WIDTH * WORLD_WIDTH) // random spawn

  const world = World.new(WORLD_WIDTH, snake_spawn_index)
  const worldWidth = world.width()

  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas')
  const ctx = canvas.getContext('2d')

  canvas.height = worldWidth * CELL_SIZE
  canvas.width = worldWidth * CELL_SIZE

  document.addEventListener('keydown', (e) => {
    switch (e.code) {
      case 'ArrowUp':
        console.log('Change dir to up')
        break
      case 'ArrowRight':
        console.log('Change dir to right')
        break
      case 'ArrowDown':
        console.log('Change dir to down')
        break
      case 'ArrowLeft':
        console.log('Change dir to left')
        break
    }
  })

  function drawWorld() {
    ctx.beginPath()

    // columns
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0)
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
    }

    // rows
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y)
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
    }

    ctx.stroke()
  }

  function drawSnake() {
    const snakeIndex = world.snake_head_index()
    const col = snakeIndex % worldWidth
    const row = Math.floor(snakeIndex / worldWidth)

    ctx.beginPath()
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
    ctx.stroke()
  }

  function paint() {
    drawWorld()
    drawSnake()
  }

  // update world
  function update() {
    const fps = 5
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height)
      // position
      world.update()
      paint()
      // synchronize - request the next animation
      requestAnimationFrame(update)
    }, 1000 / fps)
  }

  paint()
  update()
})
