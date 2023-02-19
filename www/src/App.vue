<template>
  <canvas
    id="canvas"
    width="1000"
    height="1000"
    ref="place"
    @click="draw"
  ></canvas>
  <div id="overlay" v-if="activePixel" :style="overlayStyle"></div>
  <div id="colorPicker" v-if="activePixel">
    <nav>
      <button
        v-for="color in Object.keys(colorOptions)"
        :key="color"
        :class="color"
        @click="changePixel(color)"
        :style="{ background: 'rgb(' + colorOptions[color] + ')' }"
      ></button>
    </nav>
  </div>
</template>

<script lang="ts">
/*
  code heavily copied from https://github.com/aschmelyun/laraplace/.
  please also watch his video @ https://youtube.com/watch?v=XSw5fFo0_pA
*/

import { defineComponent, reactive, ref } from "vue";

export default defineComponent({
  name: "App",
  setup() {
    const ws = new WebSocket("ws://http://95.111.231.242:9000/ws/grid/");

    const colorOptions = {
      red: "231, 76, 60",
      orange: "230, 126, 34",
      yellow: "241, 196, 15",
      green: "46, 204, 113",
      blue: "52, 152, 219",
      purple: "155, 89, 182",
      white: "236, 240, 241",
      black: "44, 62, 80",
    };

    ws.onmessage = (e) => {
      let ctx = (
        document.getElementById("canvas") as HTMLCanvasElement
      ).getContext("2d");
      if (!ctx) return;

      let data = JSON.parse(e.data);
      for (let i = 0; i < data.length; i++) {
        let pixel = data[i];
        ctx.fillStyle = `rgb(${Object.values(colorOptions)[pixel.color]})`;
        ctx.fillRect(pixel.x, pixel.y, 1, 1);
      }
    };

    setTimeout(() => {
      ws.send(`{"x": -1, "y": -1, "color": 0}`);
    }, 50);

    return {
      activePixel: ref(""),
      colorOptions,
      ws,
    };
  },
  methods: {
    draw(e: MouseEvent) {
      let x = Math.floor(e.pageX / 16);
      let y = Math.floor(e.pageY / 16);

      this.overlayStyle.top = `${y * 16}px`;
      this.overlayStyle.left = `${x * 16}px`;

      this.activePixel = `${x}:${y}`;
    },
    changePixel(color: string) {
      let ctx = (
        document.getElementById("canvas") as HTMLCanvasElement
      ).getContext("2d");
      if (!ctx) return;

      ctx.fillStyle = `rgb(${this.colorOptions[color]})`;

      let [x, y] = this.activePixel.split(":");
      ctx.fillRect(parseInt(x), parseInt(y), 1, 1);
      this.ws.send(
        `{"x":${x},"y":${y},"color":${Object.keys(this.colorOptions).indexOf(
          color
        )}}`
      );
    },
  },
  data() {
    return {
      overlayStyle: reactive({
        top: "0px",
        left: "0px",
      }),
    };
  },
});
</script>

<style>
body {
  margin: 0;
}
#canvas {
  image-rendering: pixelated;
  transform-origin: top left;
  transform: scale(16);
}
#overlay {
  position: absolute;
  width: 14px;
  height: 14px;
  z-index: 1;
  border: 1px solid #000000;
}
#colorPicker {
  background-color: #fff;
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  padding: 1rem;
  text-align: center;
  z-index: 2;
  border-top: 1px solid #000;
}
#colorPicker p {
  margin: 0 0 1rem;
  font-family: Arial, Helvetica, sans-serif;
}
#colorPicker button {
  width: 4rem;
  height: 2rem;
  border: 2px solid #000;
  border-radius: 0.5rem;
  margin: 0 0.25rem;
}
#colorPicker button:hover {
  cursor: pointer;
  opacity: 0.75;
}
</style>
