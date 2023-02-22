import express from "express";
import http from "http";
import WebSocket from "ws";
import dotenv from "dotenv";
import { Point } from "./pointModel";
import { connect } from "mongoose";

const app = express();

dotenv.config();

const server = http.createServer(app);

const wss = new WebSocket.Server({ server });

wss.on("connection", async (ws: WebSocket) => {
  ws.on("message", async (message: string) => {
    let json;

    try {
      json = JSON.parse(message);
    } catch (e) {
      return;
    }

    const exists = await Point.exists({
      x: json.x,
      y: json.y,
    });
    if (exists) {
      await Point.updateOne({ x: json.x, y: json.y }, { color: json.color });
    } else {
      const point = new Point(json);
      await point.save();
    }
    const existingPoint = await Point.findOne({ x: json.x, y: json.y });
    let point = {
      x: existingPoint?.x,
      y: existingPoint?.y,
      color: existingPoint?.color,
    };
    broadcast(JSON.stringify([point]));
  });
  
  for (let i = 0; i < 4000 / 100; i++) {
    for (let j = 0; j < 4000 / 1000; j++) {
      const points = await getPointsRange(i * 100, i * 100 + 100, j * 1000, j * 1000 + 1000);
      ws.send(JSON.stringify(points));
    }
  }
});

server.listen(process.env.PORT, async () => {
  console.log(`Server started on port ${process.env.PORT}`);
  await connect(process.env.MONGO_URI || "").then(() => {
    console.log("Connected to MongoDB");
  });
});

const getPoints = async () => {
  const points = await Point.find({});
  const pointsArray = [];

  for (let point of points) {
    pointsArray.push({
      x: point.x,
      y: point.y,
      color: point.color,
    });
  }

  return pointsArray;
};

const getPointsRange = async (x1: number, x2: number, y1: number, y2: number) => {
  const points = await Point.find({
    x: { $gte: x1, $lte: x2 },
    y: { $gte: y1, $lte: y2 },
  });
  const pointsArray = [];

  for (let point of points) {
    pointsArray.push({
      x: point.x,
      y: point.y,
      color: point.color,
    });
  }

  return pointsArray;
};

function broadcast(msg: any) {
  wss.clients.forEach(function each(client) {
    client.send(msg);
  });
}
