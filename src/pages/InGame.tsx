import { useContext, useEffect } from "react";
import { Grid, Page } from "../components";
import { TetrisContext, Tile } from "../main";
import tetrisImg from "../assets/tetris.png";
import backSvg from "../assets/back.svg";
import { Link } from "react-router-dom";

export default function InGame() {
  const tetris = useContext(TetrisContext);
  useEffect(() => {
    const onKeyPress = (ev: KeyboardEvent) => {
      console.log(ev.code);
    };
    document.addEventListener("keydown", onKeyPress);
    return () => document.removeEventListener("keydown", onKeyPress);
  }, []);
  return (
    <Page>
      <div className="flex items-center justify-center w-screen h-screen gap-6">
        <Grid grid={tetris.board} />
        <div className="flex flex-col items-end justify-center gap-3">
          <img src={tetrisImg} alt="tetris logo" width={"150px"} />
          <div
            className="flex flex-col justify-center items-center w-[150px] h-[150px] rounded-md"
            style={{ background: Tile.EMPTY_DARK }}
          >
            <Grid
              grid={[
                [
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                ],
                [
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                ],
                [
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                ],
                [
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                  Tile.EMPTY_DARK,
                ],
              ]}
            />
          </div>
          <Link to=".." relative="path">
            <img
              className="w-12 h-12 p-2 rounded-md"
              style={{ background: Tile.EMPTY_DARK }}
              src={backSvg}
              alt="back button"
            />
          </Link>
        </div>
      </div>
    </Page>
  );
}
