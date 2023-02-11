import "../style.css";

import RenderingPipeline from "./rendering-pipeline";
import MouseHandler from "./mouse-handler";
import { RenderingPipelineSettings } from "./rendering-pipeline/settings";

const canvas = document.getElementById("gameCanvas") as HTMLCanvasElement;
const gl = canvas.getContext("webgl")!;

const renderingPipelineSettings = new RenderingPipelineSettings();
const renderingPipeline = new RenderingPipeline(gl, renderingPipelineSettings);

const render = () => {
  gl.viewport(0, 0, canvas.width, canvas.height);

  renderingPipeline.render();
};

const mouseHandler = new MouseHandler(
  renderingPipeline,
  renderingPipelineSettings
);

renderingPipelineSettings.addPropertyChangedEventHandler(() => render());

const resizeCanvas = () => {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  renderingPipelineSettings.aspectRatio = canvas.width / canvas.height;
};

window.addEventListener("resize", () => resizeCanvas());
window.addEventListener("load", () => {
  resizeCanvas();
  mouseHandler.start();
});
canvas.addEventListener("contextmenu", (event) => {
  event.preventDefault();
});
