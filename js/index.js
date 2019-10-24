(async function () {
  const module = await import('../pkg/index.js');
  const memory = (await import('../pkg/index_bg.wasm')).memory;
  const COLORS = {
      [module.State.Tree]: '#2e913a',
      [module.State.Burning]: '#ff2b23',
      [module.State.Empty]: '#000000'
  }
  const WIDTH = 500;
  const HEIGHT = 500;

  function init () {
    const forest = module.Forest.new(WIDTH, HEIGHT, .02, .00001);

    const canvas = document.getElementById('forest-canvas');
    canvas.width = WIDTH;
    canvas.height = HEIGHT;

    const context2d = canvas.getContext('2d');

    const renderLoop = () => {
      console.time('tick');
      forest.tick();
      console.timeEnd('tick');

      const patchesPtr = forest.patches();
      const patches = new Uint8Array(memory.buffer, patchesPtr, WIDTH * HEIGHT);
      console.time('drawPatches');
      drawPatches(patches, context2d);
      console.timeEnd('drawPatches');

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
  }

  function drawPatches (patches, ctx) {
    for (let row = 0; row < HEIGHT; row++) {
      for (let col = 0; col < WIDTH; col++) {
          const idx = row * WIDTH + col;
          ctx.fillStyle = COLORS[patches[idx]];
          ctx.fillRect(col,row,1,1);
      }
    }
  }

  init();

})();

