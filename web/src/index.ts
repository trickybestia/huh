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
    const onMouseMove = (event: MouseEvent) => {
      wasm.on_drag(
        event.clientX,
        event.clientY,
        event.movementX,
        event.movementY
      );
    };

    window.addEventListener("contextmenu", (event) => event.preventDefault());
    window.addEventListener("resize", () => resizeCanvas());
    window.addEventListener("click", (event) => {
      wasm.on_click(event.clientX, event.clientY);
    });
    window.addEventListener("mousedown", (event) => {
      if (event.button === 2) {
        window.addEventListener("mousemove", onMouseMove);
      }
    });
    window.addEventListener("mouseup", (event) => {
      if (event.button === 2) {
        window.removeEventListener("mousemove", onMouseMove);
      }
    });
    window.addEventListener("wheel", (event) => {
      wasm.on_wheel(event.deltaY);
    });

    resizeCanvas();

    window.requestAnimationFrame(onRepaint);
  })
  .catch((error) => console.error(error));
