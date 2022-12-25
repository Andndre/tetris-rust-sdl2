import { useState } from "react";
import { pickTemplate, Template, Tile, Vec2 } from "../main";

export default () => {
  const [currentOffset, setCurrentOffset] = useState<Vec2>({ x: 4, y: 2 });
  const [board, setBoard] = useState<string[][]>(
    Array.from(Array(20), (_) => Array(10).fill(Tile.EMPTY_DARK))
  );
  const [score, setScore] = useState(0);
  const [next, setNext] = useState<Template>(pickTemplate());
  const [current, setCurrent] = useState<Template>(pickTemplate());
  const [prevOffset, setPrevOffset] = useState<Vec2>({ x: 4, y: 2 });

  function resetGame() {
    setScore(0);
    setBoard(Array.from(Array(20), (_) => Array(10).fill(Tile.EMPTY_DARK)));
    setCurrentOffset({ x: 5, y: 3 });
  }

  function move(offset: Vec2) {
    currentOffset.x += offset.x;
    currentOffset.y += offset.y;
    setCurrentOffset({ x: currentOffset.x, y: currentOffset.y });
    current.offsets.forEach((e) => {
      board[e.y + currentOffset.y - offset.y][
        e.x + currentOffset.x - offset.x
      ] = Tile.EMPTY_DARK;
      board[e.y + currentOffset.y][e.x + currentOffset.x] = current.type;
    });
    setBoard(board);
  }

  function hardDrop() {
    let min = 0;
    current.offsets.forEach((offset) => {
      const current = offset.y + currentOffset.y;
      min = 20 - current;
      for (let i = current + 1; i < 20; i++) {
        if (board[i][offset.x + currentOffset.x] !== Tile.EMPTY_DARK) {
          min = Math.min(min, i);
        }
      }
    });

    current.offsets.forEach((offset) => {
      board[offset.y + currentOffset.y + min][offset.x] = current.type;
    });

    setBoard(board);

    return min;
  }

  function lineClear() {
    // todo
  }

  function spawnPiece() {
    const newNext = pickTemplate();
    const newCurrent = next;
    setNext(newNext);
    setCurrent(newCurrent);
    setCurrentOffset({ x: 5, y: 3 });
  }

  return {
    currentOffset,
    setCurrentOffset,
    prevOffset,
    setPrevOffset,
    board,
    setBoard,
    score,
    setScore,
    resetGame,
    current,
    setCurrent,
    next,
    setNext,
    hardDrop,
    lineClear,
    spawnPiece,
    move,
  };
};
