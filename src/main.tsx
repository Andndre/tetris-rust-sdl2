import React, { Dispatch, SetStateAction } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import useTetris from "./hooks/useTetris";
import "./style.css";

export type Vec2 = {
  x: number;
  y: number;
};

export enum Tile {
  EMPTY_DARK = "#1D2D36",
  EMPTY_LIGHT = "#A9B0B5",
  T = "#A238E3",
  L = "#46D13A",
  J = "#3749E8",
  Z = "#D1953A",
  S = "#DF2C2C",
  I = "#34A5E5",
  SQUARE = "#D4E13C",
}

export type Template = {
  offsets: Vec2[];
  type: Tile;
};

export type Settings = {
  theme: "light" | "dark";
  setTheme: Dispatch<SetStateAction<"light" | "dark">>;
  primary: string;
};

export const SettingsContext = React.createContext({} as Settings);

export const templates: Template[] = [
  {
    offsets: [
      { x: 0, y: -2 },
      { x: 0, y: -1 },
      { x: 0, y: 0 },
      { x: 0, y: 1 },
    ],
    type: Tile.I,
  },
  {
    offsets: [
      { x: 0, y: -1 },
      { x: 0, y: 0 },
      { x: 0, y: 1 },
      { x: -1, y: 1 },
    ],
    type: Tile.J,
  },
  {
    offsets: [
      { x: 0, y: -1 },
      { x: 0, y: 0 },
      { x: 0, y: 1 },
      { x: 1, y: 1 },
    ],
    type: Tile.L,
  },
];

export const direction = {
  UP: { x: 0, y: -1 } as Vec2,
  DOWN: { x: 0, y: 1 } as Vec2,
  LEFT: { x: -1, y: 0 } as Vec2,
  RIGHT: { x: 1, y: 0 } as Vec2,
} as const;

export function pickTemplate() {
  return templates[Math.floor(Math.random() * templates.length)];
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
