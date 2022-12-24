import { useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { SettingsContext, TetrisContext, Tile } from "./main";
import { Home, InGame } from "./pages";

function App() {
  const [theme, setTheme] = useState<"light" | "dark">("dark");
  return (
    <SettingsContext.Provider value={{ theme, setTheme, primary: "#34A5E5" }}>
      <TetrisContext.Provider
        value={{
          board: Array.from(Array(20), (_) => Array(10).fill(Tile.EMPTY_DARK)),
          score: 0,
        }}
      >
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/play" element={<InGame />} />
          </Routes>
        </BrowserRouter>
      </TetrisContext.Provider>
    </SettingsContext.Provider>
  );
}

export default App;
