<template>
  <canvas
    id="canvas"
    width="4000"
    height="2000"
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
        :style="{ background: '' + colorOptions[color] + '' }"
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
    const ws = new WebSocket("wss://place.angelsflyinhell.dev/");

    const colorOptions = {
      black: "#000000",
      darkBlue: "#353658",
      grey: "#686b72",
      grey2: "#8b97b6",
      grey3: "#c5cddb",
      white: "#ffffff",
      lightBlue: "#5ee9e9",
      blue: "#2890dc",
      darkBlue2: "#1831a7",
      darkGreen: "#053239",
      darkGreen2: "#005f41",
      green: "#08b23b",
      green2: "#47f641",
      yellow: "#e8ff75",
      orange: "#fbbe82",
      orange2: "#de9751",
      brown: "#b66831",
      brown2: "#8a4926",
      brown3: "#461c14",
      darkBrown: "#1e090d",
      darkRed: "#720d0d",
      brown4: "#813704",
      red: "#da2424",
      orange3: "#ef6e10",
      orange4: "#ecab11",
      yellow2: "#ece910",
      pink: "#f78d8d",
      pink2: "#f94e6d",
      pink3: "#c12458",
      magenta: "#841252",
      purple: "#3d083b",
    };

    // print the colorOptions array

    ws.onmessage = (e) => {
      let ctx = (
        document.getElementById("canvas") as HTMLCanvasElement
      ).getContext("2d");
      if (!ctx) return;

      let data = JSON.parse(e.data);
      for (let i = 0; i < data.length; i++) {
        let pixel = data[i];
        ctx.fillStyle = `${Object.values(colorOptions)[pixel.color]}`;
        ctx.fillRect(pixel.x, pixel.y, 1, 1);
      }
    };

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

      ctx.fillStyle = `${this.colorOptions[color]}`;

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
  width: 100%;
  padding-bottom: 2rem;
  padding-top: 2rem;
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
