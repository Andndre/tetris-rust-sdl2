import React, { Dispatch, SetStateAction } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";

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

export type Settings = {
  theme: "light" | "dark";
  setTheme: Dispatch<SetStateAction<"light" | "dark">>;
  primary: string;
};

export const SettingsContext = React.createContext({} as Settings);

export type Tetris = {
  board: string[][];
  score: number;
};

export const TetrisContext = React.createContext({} as Tetris);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
