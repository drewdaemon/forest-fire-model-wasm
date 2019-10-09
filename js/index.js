async function init () {
  const module = await import('../pkg/index.js');
  const width = 100, height = 50;

  const forest = module.Forest.new(width, height, .02, .00001);

  const pre = document.getElementById("forest-canvas");

  // const patchesPtr = forest.patches();

  // const memory = (await import('../pkg/index_bg.wasm')).memory;
  // const patches = new Uint8Array(memory.buffer, patchesPtr, width * height);


  const renderLoop = () => {
    pre.textContent = forest.render();
    forest.tick();

    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
}

init();