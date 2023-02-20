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

    broadcast(JSON.stringify(await getPoints()));
  });
  ws.send(JSON.stringify(await getPoints()));
});

server.listen(process.env.PORT, async () => {
  console.log(`Server started on port ${process.env.PORT}`);
  await connect(process.env.MONGO_URI || "");
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

function broadcast(msg: any) {
  wss.clients.forEach(function each(client) {
    client.send(msg);
  });
}
