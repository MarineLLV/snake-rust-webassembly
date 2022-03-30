import init, { World, Direction } from 'snake_game'
import { random } from './utils/random'

init().then((wasm) => {
  const CELL_SIZE = 10
  const WORLD_WIDTH = 8
  const snake_spawn_index = random(WORLD_WIDTH * WORLD_WIDTH) // random spawn

  const world = World.new(WORLD_WIDTH, snake_spawn_index)
  const worldWidth = world.width()

  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas')
  const ctx = canvas.getContext('2d')

  canvas.height = worldWidth * CELL_SIZE
  canvas.width = worldWidth * CELL_SIZE

  document.addEventListener('keydown', (e) => {
    switch (e.code) {
      case 'ArrowUp':
        world.change_snake_dir(Direction.Up)
        break
      case 'ArrowRight':
        world.change_snake_dir(Direction.Right)
        break
      case 'ArrowDown':
        world.change_snake_dir(Direction.Down)
        break
      case 'ArrowLeft':
        world.change_snake_dir(Direction.Left)
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
    // access snake cells
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length(),
    )

    snakeCells.forEach((cellIndex, i) => {
      const col = cellIndex % worldWidth
      const row = Math.floor(cellIndex / worldWidth)

      ctx.fillStyle = i === 0 ? '#7878db' : '#000000'

      ctx.beginPath()
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
    })

    ctx.stroke()
  }

  function drawReward() {
    const rewardIndex = world.reward_cell()
    const col = rewardIndex % worldWidth
    const row = Math.floor(rewardIndex / worldWidth)

    ctx.beginPath()
    ctx.fillStyle = '#FF0000'
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)

    ctx.stroke()
  }

  function paint() {
    drawWorld()
    drawSnake()
    drawReward()
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
