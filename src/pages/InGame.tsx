import { useEffect } from "react";
import { Board, Next, Page } from "../components";
import { direction, Tile } from "../main";
import tetrisImg from "../assets/tetris.png";
import backSvg from "../assets/back.svg";
import { Link } from "react-router-dom";
import useTetris from "../hooks/useTetris";

export default () => {
  const { move, next, current, currentOffset, board } = useTetris();
  useEffect(() => {
    const onKeyPress = (ev: KeyboardEvent) => {
      console.log(ev.code);
      switch (ev.code) {
        case "ArrowDown":
          move(direction.DOWN);
          break;
        case "ArrowUp":
          move(direction.UP);
          break;
        case "ArrowLeft":
          move(direction.LEFT);
          break;
        case "ArrowRight":
          move(direction.RIGHT);
          break;
      }
    };
    document.addEventListener("keydown", onKeyPress);
    return () => document.removeEventListener("keydown", onKeyPress);
  }, []);

  return (
    <Page>
      <div className="flex items-center justify-center w-screen h-screen gap-6">
        <Board current={current} currentOffset={currentOffset} board={board} />
        <div className="flex flex-col items-end justify-center gap-3">
          <img src={tetrisImg} alt="tetris logo" width={"150px"} />
          <div
            className="flex flex-col justify-center items-center w-[150px] h-[150px] rounded-md"
            style={{ border: Tile.EMPTY_DARK + " 2px solid" }}
          >
            <Next next={next} />
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
};
