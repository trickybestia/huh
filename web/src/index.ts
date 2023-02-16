import "../style.css";

import("logic")
  .then((wasm) => {
    const canvas = document.getElementById("gameCanvas") as HTMLCanvasElement;
    const gl = canvas.getContext("webgl")!;

    const resizeCanvas = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;

      gl.viewport(0, 0, canvas.width, canvas.height);
    };

    wasm.create_engine(gl);

    const onRepaint = () => {
      wasm.render();

      window.requestAnimationFrame(onRepaint);
    };

    window.addEventListener("contextmenu", (event) => event.preventDefault());
    window.addEventListener("resize", () => resizeCanvas());
    window.addEventListener("mousedown", (event) => {
      if (event.button === 0) {
        wasm.on_mouse_down(event.clientX, event.clientY);
      }
    });
    window.addEventListener("mouseup", (event) => {
      if (event.button === 0) {
        wasm.on_mouse_up(event.clientX, event.clientY);
      }
    });
    window.addEventListener("mousemove", (event) =>
      wasm.on_mouse_move(event.clientX, event.clientY)
    );

    resizeCanvas();

    window.requestAnimationFrame(onRepaint);
  })
  .catch((error) => console.error(error));
