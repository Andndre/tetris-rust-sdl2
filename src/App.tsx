import { useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { SettingsContext } from "./main";
import { Home, InGame } from "./pages";

export default () => {
  const [theme, setTheme] = useState<"light" | "dark">("dark");
  return (
    <SettingsContext.Provider value={{ theme, setTheme, primary: "#34A5E5" }}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/play" element={<InGame />} />
        </Routes>
      </BrowserRouter>
    </SettingsContext.Provider>
  );
};
